//! Original Go file: `storage.bridge.go`
//! Package: `main`; LOC: 144; SHA256: `d9cf8f51ea19b9a0b1584bb52cc5311d783b32c9b648f6b7eee206a6024860bf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "storage.bridge.go", package: "main", go_loc: 144, functions: 5, types: 0, sha256: "d9cf8f51ea19b9a0b1584bb52cc5311d783b32c9b648f6b7eee206a6024860bf" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/storage",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("BridgeStoragePoolInit", "", 15),
    ("BridgeStoragePoolForGroup", "", 43),
    ("DebridgeAllFSHandlerFromGroup", "", 74),
    ("BridgeFSHandlerToGroup", "", 110),
    ("DebridgeFSHandlerFromGroup", "", 122),
];

pub async fn bridgestoragepoolinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.bridge.go", function: "BridgeStoragePoolInit" })
}

pub async fn bridgestoragepoolforgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.bridge.go", function: "BridgeStoragePoolForGroup" })
}

pub async fn debridgeallfshandlerfromgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.bridge.go", function: "DebridgeAllFSHandlerFromGroup" })
}

pub async fn bridgefshandlertogroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.bridge.go", function: "BridgeFSHandlerToGroup" })
}

pub async fn debridgefshandlerfromgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.bridge.go", function: "DebridgeFSHandlerFromGroup" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
