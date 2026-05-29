//! Original Go file: `mod/iot/handlerManager.go`
//! Package: `iot`; LOC: 222; SHA256: `ced524010a4ce07d93a723400e34551c4428db34a02abcef44975dab404032df`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/handlerManager.go", package: "iot", go_loc: 222, functions: 11, types: 1, sha256: "ced524010a4ce07d93a723400e34551c4428db34a02abcef44975dab404032df" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Manager", "struct", 21),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewIoTManager", "", 27),
    ("RegisterHandler", "m *Manager", 42),
    ("HandleScannerList", "m *Manager", 57),
    ("GetDeviceByID", "m *Manager", 69),
    ("HandleIconLoad", "m *Manager", 79),
    ("HandleExecute", "m *Manager", 100),
    ("HandleGetDeviceStatus", "m *Manager", 146),
    ("HandleScanning", "m *Manager", 175),
    ("HandleListing", "m *Manager", 184),
    ("GetCachedDeviceList", "m *Manager", 194),
    ("ScanDevices", "m *Manager", 202),
];

pub async fn newiotmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "NewIoTManager" })
}

pub async fn manager_registerhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.RegisterHandler" })
}

pub async fn manager_handlescannerlist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.HandleScannerList" })
}

pub async fn manager_getdevicebyid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.GetDeviceByID" })
}

pub async fn manager_handleiconload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.HandleIconLoad" })
}

pub async fn manager_handleexecute(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.HandleExecute" })
}

pub async fn manager_handlegetdevicestatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.HandleGetDeviceStatus" })
}

pub async fn manager_handlescanning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.HandleScanning" })
}

pub async fn manager_handlelisting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.HandleListing" })
}

pub async fn manager_getcacheddevicelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.GetCachedDeviceList" })
}

pub async fn manager_scandevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/handlerManager.go", function: "Manager.ScanDevices" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
