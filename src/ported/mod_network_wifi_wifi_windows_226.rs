//! Original Go file: `mod/network/wifi/wifi_windows.go`
//! Package: `wifi`; LOC: 198; SHA256: `9db2881018dc98abf0a02ca7f7a98d8e71388823a861cad8da04a3fc336d6c7b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/wifi/wifi_windows.go", package: "wifi", go_loc: 198, functions: 7, types: 0, sha256: "9db2881018dc98abf0a02ca7f7a98d8e71388823a861cad8da04a3fc336d6c7b" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "log",
    "os/exec",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SetInterfacePower", "w *WiFiManager", 19),
    ("GetInterfacePowerStatuts", "w *WiFiManager", 23),
    ("ScanNearbyWiFi", "w *WiFiManager", 27),
    ("GetWirelessInterfaces", "w *WiFiManager", 127),
    ("ConnectWiFi", "w *WiFiManager", 155),
    ("GetConnectedWiFi", "w *WiFiManager", 160),
    ("RemoveWifi", "w *WiFiManager", 196),
];

pub async fn wifimanager_setinterfacepower(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.SetInterfacePower" })
}

pub async fn wifimanager_getinterfacepowerstatuts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.GetInterfacePowerStatuts" })
}

pub async fn wifimanager_scannearbywifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.ScanNearbyWiFi" })
}

pub async fn wifimanager_getwirelessinterfaces(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.GetWirelessInterfaces" })
}

pub async fn wifimanager_connectwifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.ConnectWiFi" })
}

pub async fn wifimanager_getconnectedwifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.GetConnectedWiFi" })
}

pub async fn wifimanager_removewifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_windows.go", function: "WiFiManager.RemoveWifi" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
