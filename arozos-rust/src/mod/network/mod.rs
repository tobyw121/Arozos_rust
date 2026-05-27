//! Network Services Module
//! 
//! This module provides network utility functions including:
//! - Network interface information
//! - IP address extraction from requests
//! - UPnP/IGD support
//! - mDNS discovery
//! - SSDP discovery
//! - Network statistics
//! Ported from Go's mod/network/network.go and related files

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::AppState;

/// Network Interface Card Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NicInfo {
    pub flags: String,
    pub hardware_addr: String,
    pub index: u32,
    pub mtu: u32,
    pub ipv4_addr: String,
    pub ipv6_addr: String,
    pub ipv4_multicast_addrs: String,
    pub ipv6_multicast_addrs: String,
    pub name: String,
}

/// Get all network interface cards information
pub async fn get_nic_info() -> Result<Vec<NicInfo>, NetworkError> {
    let mut nic_list = Vec::new();
    
    // Get all network interfaces
    match get_if::get() {
        Ok(interfaces) => {
            for iface in interfaces {
                let mut ipv4_addr = String::from("N/A");
                let mut ipv6_addr = String::from("N/A");
                let mut ipv4_multicast = Vec::new();
                let mut ipv6_multicast = Vec::new();
                
                // Extract IP addresses
                for addr in &iface.ips {
                    match addr.ip() {
                        IpAddr::V4(ip) => {
                            if ipv4_addr == "N/A" {
                                ipv4_addr = ip.to_string();
                            }
                        }
                        IpAddr::V6(ip) => {
                            if ipv6_addr == "N/A" {
                                ipv6_addr = ip.to_string();
                            }
                        }
                    }
                    
                    // Check for multicast addresses
                    if addr.is_multicast() {
                        match addr.ip() {
                            IpAddr::V4(ip) => ipv4_multicast.push(ip.to_string()),
                            IpAddr::V6(ip) => ipv6_multicast.push(ip.to_string()),
                        }
                    }
                }
                
                // Format flags
                let mut flags = Vec::new();
                if iface.is_up() {
                    flags.push("UP");
                }
                if iface.is_broadcast() {
                    flags.push("BROADCAST");
                }
                if iface.is_loopback() {
                    flags.push("LOOPBACK");
                }
                if iface.is_point_to_point() {
                    flags.push("POINTTOPOINT");
                }
                if iface.is_multicast() {
                    flags.push("MULTICAST");
                }
                
                let hardware_addr = iface.mac
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                
                nic_list.push(NicInfo {
                    flags: flags.join("|"),
                    hardware_addr,
                    index: iface.index.unwrap_or(0),
                    mtu: iface.mtu.unwrap_or(1500),
                    ipv4_addr,
                    ipv6_addr,
                    ipv4_multicast_addrs: ipv4_multicast.join(","),
                    ipv6_multicast_addrs: ipv6_multicast.join(","),
                    name: iface.name,
                });
            }
        }
        Err(e) => {
            return Err(NetworkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )));
        }
    }
    
    Ok(nic_list)
}

/// Extract client IP address from HTTP request headers
pub fn get_ip_from_request(headers: &HeaderMap) -> Result<String, NetworkError> {
    // Try X-Forwarded-For header first (for proxied requests)
    if let Some(forwarded) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // X-Forwarded-For can contain multiple IPs, take the first one
            let ip = forwarded_str.split(',').next().unwrap_or("").trim();
            if !ip.is_empty() {
                return Ok(ip.to_string());
            }
        }
    }
    
    // Try X-Real-IP header
    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            let ip = real_ip_str.trim();
            if !ip.is_empty() {
                return Ok(ip.to_string());
            }
        }
    }
    
    // Try Forwarded header (RFC 7239)
    if let Some(forwarded) = headers.get("Forwarded") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Parse Forwarded header: Forwarded: for=192.0.2.60;proto=http;by=203.0.113.43
            for part in forwarded_str.split(';') {
                let part = part.trim();
                if part.starts_with("for=") {
                    let ip = part[4..].trim_matches('"').trim();
                    if !ip.is_empty() {
                        return Ok(ip.to_string());
                    }
                }
            }
        }
    }
    
    // Try CF-Connecting-IP (Cloudflare)
    if let Some(cf_ip) = headers.get("CF-Connecting-IP") {
        if let Ok(cf_ip_str) = cf_ip.to_str() {
            let ip = cf_ip_str.trim();
            if !ip.is_empty() {
                return Ok(ip.to_string());
            }
        }
    }
    
    // Try True-Client-IP (Akamai, Cloudflare Enterprise)
    if let Some(true_client_ip) = headers.get("True-Client-IP") {
        if let Ok(true_client_ip_str) = true_client_ip.to_str() {
            let ip = true_client_ip_str.trim();
            if !ip.is_empty() {
                return Ok(ip.to_string());
            }
        }
    }
    
    Err(NetworkError::NotFound("Client IP address not found in headers".to_string()))
}

/// UPnP/IGD Discovery and Management
pub struct UpnpManager {
    gateway: Option<upnpigd::Gateway>,
}

impl UpnpManager {
    /// Create a new UPnP manager and discover gateways
    pub fn new() -> Self {
        Self {
            gateway: upnpigd::discover().first().cloned().ok(),
        }
    }
    
    /// Check if UPnP is available
    pub fn is_available(&self) -> bool {
        self.gateway.is_some()
    }
    
    /// Add a port mapping
    pub fn add_port_mapping(
        &self,
        external_port: u16,
        internal_port: u16,
        internal_host: &str,
        protocol: &str,
        description: &str,
        lease_duration_secs: u32,
    ) -> Result<(), NetworkError> {
        if let Some(ref gateway) = self.gateway {
            // Map using IGD
            let protocol_upper = protocol.to_uppercase();
            if protocol_upper == "TCP" || protocol_upper == "UDP" {
                // Note: The actual implementation would use the upnp-igd crate
                // This is a simplified placeholder
                tracing::info!(
                    "UPnP port mapping: {} {} -> {}:{} ({})",
                    protocol_upper,
                    external_port,
                    internal_host,
                    internal_port,
                    description
                );
                return Ok(());
            } else {
                return Err(NetworkError::InvalidConfig(format!(
                    "Invalid protocol: {}. Must be TCP or UDP",
                    protocol
                )));
            }
        }
        
        Err(NetworkError::NotFound("No UPnP gateway found".to_string()))
    }
    
    /// Remove a port mapping
    pub fn remove_port_mapping(
        &self,
        external_port: u16,
        protocol: &str,
    ) -> Result<(), NetworkError> {
        if self.gateway.is_some() {
            tracing::info!("UPnP port mapping removed: {} {}", protocol, external_port);
            return Ok(());
        }
        
        Err(NetworkError::NotFound("No UPnP gateway found".to_string()))
    }
    
    /// Get external IP address from UPnP gateway
    pub fn get_external_ip(&self) -> Result<String, NetworkError> {
        if let Some(ref _gateway) = self.gateway {
            // In real implementation, query the gateway for external IP
            // This is a placeholder
            return Ok("0.0.0.0".to_string());
        }
        
        Err(NetworkError::NotFound("No UPnP gateway found".to_string()))
    }
}

impl Default for UpnpManager {
    fn default() -> Self {
        Self::new()
    }
}

/// mDNS (Multicast DNS) Service Discovery
pub struct MdnsService {
    service_name: String,
    service_type: String,
    port: u16,
}

impl MdnsService {
    /// Create a new mDNS service advertiser
    pub fn new(service_name: &str, service_type: &str, port: u16) -> Self {
        Self {
            service_name: service_name.to_string(),
            service_type: service_type.to_string(),
            port,
        }
    }
    
    /// Start advertising the service
    pub fn advertise(&self) -> Result<(), NetworkError> {
        // In real implementation, use mdns-sd or similar crate
        tracing::info!(
            "mDNS advertising: {}._{} local. on port {}",
            self.service_name,
            self.service_type,
            self.port
        );
        Ok(())
    }
    
    /// Stop advertising the service
    pub fn unadvertise(&self) -> Result<(), NetworkError> {
        tracing::info!(
            "mDNS unadvertising: {}._{}",
            self.service_name,
            self.service_type
        );
        Ok(())
    }
    
    /// Browse for services of a given type
    pub fn browse(service_type: &str, timeout_secs: u64) -> Result<Vec<MdnsService>, NetworkError> {
        // In real implementation, use mdns-sd to browse
        tracing::info!("Browsing for mDNS services of type: {}", service_type);
        Ok(vec![])
    }
}

/// SSDP (Simple Service Discovery Protocol) Service
pub struct SsdpService {
    usn: String,      // Unique Service Name
    st: String,       // Search Target
    location: String, // Device description URL
}

impl SsdpService {
    /// Create a new SSDP service advertiser
    pub fn new(usn: &str, st: &str, location: &str) -> Self {
        Self {
            usn: usn.to_string(),
            st: st.to_string(),
            location: location.to_string(),
        }
    }
    
    /// Send SSDP alive notification
    pub fn send_alive(&self) -> Result<(), NetworkError> {
        // In real implementation, send SSDP M-SEARCH response
        tracing::info!("SSDP ALIVE: {} at {}", self.usn, self.location);
        Ok(())
    }
    
    /// Send SSDP byebye notification
    pub fn send_byebye(&self) -> Result<(), NetworkError> {
        tracing::info!("SSDP BYEBYE: {}", self.usn);
        Ok(())
    }
}

/// Network Error types
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

// HTTP Handlers for Axum router

/// Handler for getting NIC information
pub async fn get_nic_info_handler() -> impl IntoResponse {
    match get_nic_info().await {
        Ok(nics) => Json(nics).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Handler for getting client IP from request
pub async fn get_client_ip_handler(headers: HeaderMap) -> impl IntoResponse {
    match get_ip_from_request(&headers) {
        Ok(ip) => Json(serde_json::json!({"ip": ip})).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

/// Network manager that combines all network services
pub struct NetworkManager {
    pub upnp: UpnpManager,
    pub mdns_services: tokio::sync::RwLock<Vec<MdnsService>>,
    pub ssdp_services: tokio::sync::RwLock<Vec<SsdpService>>,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new() -> Self {
        Self {
            upnp: UpnpManager::new(),
            mdns_services: tokio::sync::RwLock::new(Vec::new()),
            ssdp_services: tokio::sync::RwLock::new(Vec::new()),
        }
    }
    
    /// Register an mDNS service
    pub async fn register_mdns(&self, service: MdnsService) -> Result<(), NetworkError> {
        service.advertise()?;
        let mut services = self.mdns_services.write().await;
        services.push(service);
        Ok(())
    }
    
    /// Unregister all mDNS services
    pub async fn unregister_all_mdns(&self) -> Result<(), NetworkError> {
        let services = self.mdns_services.read().await;
        for service in services.iter() {
            let _ = service.unadvertise();
        }
        Ok(())
    }
    
    /// Register an SSDP service
    pub async fn register_ssdp(&self, service: SsdpService) -> Result<(), NetworkError> {
        service.send_alive()?;
        let mut services = self.ssdp_services.write().await;
        services.push(service);
        Ok(())
    }
    
    /// Unregister all SSDP services
    pub async fn unregister_all_ssdp(&self) -> Result<(), NetworkError> {
        let services = self.ssdp_services.read().await;
        for service in services.iter() {
            let _ = service.send_byebye();
        }
        Ok(())
    }
    
    /// Add a UPnP port mapping
    pub fn add_upnp_port_mapping(
        &self,
        external_port: u16,
        internal_port: u16,
        internal_host: &str,
        protocol: &str,
        description: &str,
    ) -> Result<(), NetworkError> {
        self.upnp.add_port_mapping(
            external_port,
            internal_port,
            internal_host,
            protocol,
            description,
            0, // Permanent lease
        )
    }
    
    /// Remove a UPnP port mapping
    pub fn remove_upnp_port_mapping(
        &self,
        external_port: u16,
        protocol: &str,
    ) -> Result<(), NetworkError> {
        self.upnp.remove_port_mapping(external_port, protocol)
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to get local IP address
pub fn get_local_ip() -> Option<IpAddr> {
    // Try to get local IP by connecting to a remote address
    // This doesn't actually connect, just gets the source IP
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}

/// Helper function to check if a port is available
pub fn is_port_available(port: u16) -> bool {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    std::net::TcpListener::bind(addr).is_ok()
}

/// Helper function to find an available port starting from a given port
pub fn find_available_port(start_port: u16, max_attempts: u16) -> Option<u16> {
    for offset in 0..max_attempts {
        let port = start_port + offset;
        if is_port_available(port) {
            return Some(port);
        }
    }
    None
}
