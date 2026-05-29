//! Original Go file: `mod/network/wifi/wifi_freebsd.go`
//! Package: `wifi`; LOC: 37; SHA256: `a96ec753d20ccdfe44df5a9f4b06ca4e5542dce5965b6821869df93ef0d2ffe0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/wifi/wifi_freebsd.go", package: "wifi", go_loc: 37, functions: 7, types: 0, sha256: "a96ec753d20ccdfe44df5a9f4b06ca4e5542dce5965b6821869df93ef0d2ffe0" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SetInterfacePower", "w *WiFiManager", 11),
    ("GetInterfacePowerStatuts", "w *WiFiManager", 15),
    ("ScanNearbyWiFi", "w *WiFiManager", 19),
    ("GetWirelessInterfaces", "w *WiFiManager", 23),
    ("ConnectWiFi", "w *WiFiManager", 27),
    ("GetConnectedWiFi", "w *WiFiManager", 31),
    ("RemoveWifi", "w *WiFiManager", 35),
];

pub async fn wifimanager_setinterfacepower(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.SetInterfacePower" })
}

pub async fn wifimanager_getinterfacepowerstatuts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.GetInterfacePowerStatuts" })
}

pub async fn wifimanager_scannearbywifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.ScanNearbyWiFi" })
}

pub async fn wifimanager_getwirelessinterfaces(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.GetWirelessInterfaces" })
}

pub async fn wifimanager_connectwifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.ConnectWiFi" })
}

pub async fn wifimanager_getconnectedwifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.GetConnectedWiFi" })
}

pub async fn wifimanager_removewifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_freebsd.go", function: "WiFiManager.RemoveWifi" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
