//! Original Go file: `mod/network/wifi/wifi_linux.go`
//! Package: `wifi`; LOC: 666; SHA256: `917c37c0095a8b3661e7304be9f5b9293e8e32ca9d33d9e5048eb3259f392bcf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/wifi/wifi_linux.go", package: "wifi", go_loc: 666, functions: 12, types: 0, sha256: "917c37c0095a8b3661e7304be9f5b9293e8e32ca9d33d9e5048eb3259f392bcf" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "os/exec",
    "path/filepath",
    "sort",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SetInterfacePower", "w *WiFiManager", 21),
    ("GetInterfacePowerStatuts", "w *WiFiManager", 37),
    ("ScanNearbyWiFi", "w *WiFiManager", 71),
    ("getSignalLevelEstimation", "w *WiFiManager", 265),
    ("GetWirelessInterfaces", "w *WiFiManager", 281),
    ("ConnectWiFi", "w *WiFiManager", 295),
    ("GetConnectedWiFi", "w *WiFiManager", 509),
    ("CheckInterfaceIsAP", "w *WiFiManager", 587),
    ("RemoveWifi", "w *WiFiManager", 605),
    ("fileInDir", "", 629),
    ("fileExists", "", 649),
    ("pkg_exists", "", 657),
];

pub async fn wifimanager_setinterfacepower(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.SetInterfacePower" })
}

pub async fn wifimanager_getinterfacepowerstatuts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.GetInterfacePowerStatuts" })
}

pub async fn wifimanager_scannearbywifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.ScanNearbyWiFi" })
}

pub async fn wifimanager_getsignallevelestimation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.getSignalLevelEstimation" })
}

pub async fn wifimanager_getwirelessinterfaces(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.GetWirelessInterfaces" })
}

pub async fn wifimanager_connectwifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.ConnectWiFi" })
}

pub async fn wifimanager_getconnectedwifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.GetConnectedWiFi" })
}

pub async fn wifimanager_checkinterfaceisap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.CheckInterfaceIsAP" })
}

pub async fn wifimanager_removewifi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "WiFiManager.RemoveWifi" })
}

pub async fn fileindir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "fileInDir" })
}

pub async fn fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "fileExists" })
}

pub async fn pkg_exists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/wifi/wifi_linux.go", function: "pkg_exists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
