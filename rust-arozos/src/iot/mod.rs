//! IoT Module for ArozOS - HDS (Home Device System) & Sonoff
//! 
//! Implements MQTT-based IoT device management for smart home devices
//! including Sonoff switches and compatible HDS devices.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::{info, warn, error, debug};

/// IoT Configuration
#[derive(Clone)]
pub struct IotConfig {
    pub mqtt_broker_url: String,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
    pub client_id: String,
    pub auto_reconnect: bool,
    pub reconnect_delay_secs: u64,
}

impl Default for IotConfig {
    fn default() -> Self {
        Self {
            mqtt_broker_url: "tcp://localhost:1883".to_string(),
            mqtt_username: None,
            mqtt_password: None,
            client_id: format!("arozos-{}", uuid::Uuid::new_v4()),
            auto_reconnect: true,
            reconnect_delay_secs: 5,
        }
    }
}

/// IoT Manager - Central hub for all IoT devices
pub struct IotManager {
    config: IotConfig,
    mqtt_client: RwLock<Option<rumqttc::AsyncClient>>,
    devices: RwLock<HashMap<String, DeviceInfo>>,
    event_tx: broadcast::Sender<IoTEvent>,
    connected: RwLock<bool>,
}

impl IotManager {
    pub fn new(config: IotConfig) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        
        Self {
            config,
            mqtt_client: RwLock::new(None),
            devices: RwLock::new(HashMap::new()),
            event_tx,
            connected: RwLock::new(false),
        }
    }

    /// Connect to MQTT broker
    pub async fn connect(&self) -> Result<(), IoTError> {
        if *self.connected.read().await {
            return Err(IoTError::AlreadyConnected);
        }

        info!("Connecting to MQTT broker: {}", self.config.mqtt_broker_url);

        // Create MQTT options
        let mut options = rumqttc::MqttOptions::new(
            self.config.client_id.clone(),
            self.config.mqtt_broker_url.trim_start_matches("tcp://"),
            1883,
        );

        if let Some(username) = &self.config.mqtt_username {
            options.set_credentials(username, self.config.mqtt_password.as_deref().unwrap_or(""));
        }

        options.set_keep_alive(std::time::Duration::from_secs(30));
        options.set_auto_reconnect(self.config.auto_reconnect);

        // Create async client
        let (client, mut eventloop) = rumqttc::AsyncClient::new(options, 10);

        // Spawn event loop handler
        let event_tx = self.event_tx.clone();
        let connected = self.connected.clone();
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        match event {
                            rumqttc::Event::Incoming(rumqttc::Packet::ConnAck(_)) => {
                                info!("MQTT connection established");
                                *connected.write().await = true;
                                let _ = event_tx.send(IoTEvent::Connected);
                            }
                            rumqttc::Event::Incoming(rumqttc::Packet::Disconnect) => {
                                warn!("MQTT disconnected");
                                *connected.write().await = false;
                                let _ = event_tx.send(IoTEvent::Disconnected);
                            }
                            rumqttc::Event::Incoming(rumqttc::Packet::Publish(pub_msg)) => {
                                debug!("Received MQTT message on topic: {}", pub_msg.topic);
                                let _ = event_tx.send(IoTEvent::MessageReceived {
                                    topic: pub_msg.topic.to_string(),
                                    payload: pub_msg.payload.to_vec(),
                                });
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("MQTT event loop error: {}", e);
                        *connected.write().await = false;
                    }
                }
            }
        });

        // Subscribe to device topics
        client.subscribe("arozos/devices/+/state", rumqttc::QoS::AtLeastOnce).await?;
        client.subscribe("arozos/devices/+/config", rumqttc::QoS::AtLeastOnce).await?;

        *self.mqtt_client.write().await = Some(client);
        info!("IoT manager connected");
        Ok(())
    }

    /// Disconnect from MQTT broker
    pub async fn disconnect(&self) -> Result<(), IoTError> {
        if let Some(client) = self.mqtt_client.write().await.take() {
            info!("Disconnecting from MQTT broker...");
            // Client will be dropped, triggering disconnect
        }
        
        *self.connected.write().await = false;
        info!("IoT manager disconnected");
        Ok(())
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    /// Register a device
    pub async fn register_device(&self, device: DeviceInfo) -> Result<(), IoTError> {
        let mut devices = self.devices.write().await;
        devices.insert(device.id.clone(), device);
        Ok(())
    }

    /// Get device info
    pub async fn get_device(&self, device_id: &str) -> Option<DeviceInfo> {
        let devices = self.devices.read().await;
        devices.get(device_id).cloned()
    }

    /// List all devices
    pub async fn list_devices(&self) -> Vec<DeviceInfo> {
        let devices = self.devices.read().await;
        devices.values().cloned().collect()
    }

    /// Send command to device
    pub async fn send_command(&self, device_id: &str, command: &DeviceCommand) -> Result<(), IoTError> {
        if !*self.connected.read().await {
            return Err(IoTError::NotConnected);
        }

        let client = self.mqtt_client.read().await;
        if let Some(client) = client.as_ref() {
            let topic = format!("arozos/devices/{}/command", device_id);
            let payload = serde_json::to_vec(command)?;
            
            client.publish(topic, rumqttc::QoS::AtLeastOnce, false, payload).await?;
            info!("Sent command to device {}: {:?}", device_id, command);
            Ok(())
        } else {
            Err(IoTError::NotConnected)
        }
    }

    /// Turn on a switch device
    pub async fn turn_on(&self, device_id: &str) -> Result<(), IoTError> {
        self.send_command(device_id, &DeviceCommand::Switch(true)).await
    }

    /// Turn off a switch device
    pub async fn turn_off(&self, device_id: &str) -> Result<(), IoTError> {
        self.send_command(device_id, &DeviceCommand::Switch(false)).await
    }

    /// Toggle a switch device
    pub async fn toggle(&self, device_id: &str) -> Result<(), IoTError> {
        self.send_command(device_id, &DeviceCommand::Toggle).await
    }

    /// Set brightness for dimmable device
    pub async fn set_brightness(&self, device_id: &str, brightness: u8) -> Result<(), IoTError> {
        self.send_command(device_id, &DeviceCommand::Brightness(brightness)).await
    }

    /// Subscribe to device events
    pub fn subscribe_events(&self) -> broadcast::Receiver<IoTEvent> {
        self.event_tx.subscribe()
    }
}

/// Device Information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub manufacturer: String,
    pub model: String,
    pub firmware_version: String,
    pub online: bool,
    pub state: DeviceState,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Device Types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Switch,
    Dimmer,
    Light,
    Sensor,
    Thermostat,
    Plug,
    Fan,
    Other(String),
}

/// Device State
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceState {
    pub powered: bool,
    pub brightness: Option<u8>,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub power_consumption: Option<f32>,
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            powered: false,
            brightness: None,
            temperature: None,
            humidity: None,
            power_consumption: None,
            custom: HashMap::new(),
        }
    }
}

/// Device Commands
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceCommand {
    Switch(bool),
    Toggle,
    Brightness(u8),
    SetTemperature(f32),
    Restart,
    FactoryReset,
    Custom(String, serde_json::Value),
}

/// IoT Events
#[derive(Clone, Debug)]
pub enum IoTEvent {
    Connected,
    Disconnected,
    DeviceRegistered(String),
    DeviceUnregistered(String),
    DeviceStateChanged(String, DeviceState),
    MessageReceived { topic: String, payload: Vec<u8> },
}

/// IoT Error Types
#[derive(Debug, thiserror::Error)]
pub enum IoTError {
    #[error("Already connected")]
    AlreadyConnected,
    #[error("Not connected")]
    NotConnected,
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("MQTT error: {0}")]
    Mqtt(#[from] rumqttc::ClientError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// ============================================================================
// Sonoff Specific Implementation
// ============================================================================

/// Sonoff Device Handler
pub struct SonoffHandler {
    mqtt_client: Arc<RwLock<Option<rumqttc::AsyncClient>>>,
}

impl SonoffHandler {
    pub fn new(mqtt_client: Arc<RwLock<Option<rumqttc::AsyncClient>>>) -> Self {
        Self { mqtt_client }
    }

    /// Parse Sonoff device state from MQTT payload
    pub fn parse_state(&self, payload: &[u8]) -> Result<DeviceState, IoTError> {
        // Sonoff typically sends JSON like: {"Switch":"on"} or {"Switch":"off"}
        #[derive(Deserialize)]
        struct SonoffPayload {
            #[serde(rename = "Switch")]
            switch: Option<String>,
            #[serde(rename = "StartTime")]
            start_time: Option<String>,
        }

        let parsed: SonoffPayload = serde_json::from_slice(payload)?;
        
        let mut state = DeviceState::default();
        if let Some(switch) = parsed.switch {
            state.powered = switch.to_lowercase() == "on";
        }

        Ok(state)
    }

    /// Create Sonoff command payload
    pub fn create_command(&self, on: bool) -> Vec<u8> {
        let cmd = if on { "on" } else { "off" };
        format!(r#"{{"Switch":"{}"}}"#, cmd).into_bytes()
    }
}

// ============================================================================
// HDS (Home Device System) Specific Implementation
// ============================================================================

/// HDS Device Handler
pub struct HdsHandler {
    mqtt_client: Arc<RwLock<Option<rumqttc::AsyncClient>>>,
}

impl HdsHandler {
    pub fn new(mqtt_client: Arc<RwLock<Option<rumqttc::AsyncClient>>>) -> Self {
        Self { mqtt_client }
    }

    /// Parse HDS device state from MQTT payload
    pub fn parse_state(&self, payload: &[u8]) -> Result<DeviceState, IoTError> {
        // HDS may use different payload formats
        #[derive(Deserialize)]
        struct HdsPayload {
            power: Option<bool>,
            brightness: Option<u8>,
            temperature: Option<f32>,
            humidity: Option<f32>,
        }

        let parsed: HdsPayload = serde_json::from_slice(payload)?;
        
        let mut state = DeviceState::default();
        if let Some(power) = parsed.power {
            state.powered = power;
        }
        state.brightness = parsed.brightness;
        state.temperature = parsed.temperature;
        state.humidity = parsed.humidity;

        Ok(state)
    }

    /// Create HDS command payload
    pub fn create_command(&self, power: Option<bool>, brightness: Option<u8>) -> Vec<u8> {
        let cmd = serde_json::json!({
            "power": power,
            "brightness": brightness,
        });
        cmd.to_string().into_bytes()
    }
}

/// Discover IoT devices on the network
pub async fn discover_devices() -> Result<Vec<DeviceInfo>, IoTError> {
    // In a full implementation, this would:
    // 1. Scan local network for devices
    // 2. Use mDNS/SSDP for discovery
    // 3. Query MQTT broker for known devices
    // 4. Return list of discovered devices
    
    info!("Discovering IoT devices...");
    
    // Placeholder - returns empty list
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iot_config_default() {
        let config = IotConfig::default();
        assert!(config.mqtt_broker_url.contains("localhost"));
        assert!(config.auto_reconnect);
    }

    #[test]
    fn test_sonoff_command() {
        let handler = SonoffHandler::new(Arc::new(RwLock::new(None)));
        
        let on_cmd = handler.create_command(true);
        let off_cmd = handler.create_command(false);
        
        assert!(String::from_utf8_lossy(&on_cmd).contains("on"));
        assert!(String::from_utf8_lossy(&off_cmd).contains("off"));
    }

    #[test]
    fn test_hds_command() {
        let handler = HdsHandler::new(Arc::new(RwLock::new(None)));
        
        let cmd = handler.create_command(Some(true), Some(50));
        let json: serde_json::Value = serde_json::from_slice(&cmd).unwrap();
        
        assert_eq!(json["power"], true);
        assert_eq!(json["brightness"], 50);
    }

    #[test]
    fn test_device_state_default() {
        let state = DeviceState::default();
        assert!(!state.powered);
        assert!(state.brightness.is_none());
    }
}
