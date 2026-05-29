//! Original Go file: `storage.go`
//! Package: `main`; LOC: 437; SHA256: `c39ee84b2ca1e52d2b3b1f9f533de91340c9bec96f2beca79fb240dd48d634d0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "storage.go", package: "main", go_loc: 437, functions: 16, types: 0, sha256: "c39ee84b2ca1e52d2b3b1f9f533de91340c9bec96f2beca79fb240dd48d634d0" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/storage",
    "imuslab.com/arozos/mod/storage/bridge",
    "log",
    "os",
    "path/filepath",
    "runtime",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("StorageInit", "", 30),
    ("LoadBaseStoragePool", "", 48),
    ("loadAlpnasNativeBaseHandlers", "", 141),
    ("storageHeartbeatTickerInit", "", 191),
    ("StoragePerformFileSystemAbstractionConnectionHeartbeat", "", 209),
    ("GroupStoragePoolInit", "", 254),
    ("LoadStoragePoolForGroup", "", 270),
    ("StoragePoolExists", "", 324),
    ("GetAllStoragePools", "", 329),
    ("GetStoragePoolByOwner", "", 341),
    ("GetFSHandlerSubpathFromVpath", "", 351),
    ("GetFsHandlerByUUID", "", 373),
    ("GetAllLoadedFsh", "", 392),
    ("RegisterStorageSettings", "", 411),
    ("CloseAllStorages", "", 425),
    ("closeAllStoragePools", "", 429),
];

pub async fn storageinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "StorageInit" })
}

pub async fn loadbasestoragepool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "LoadBaseStoragePool" })
}

pub async fn loadalpnasnativebasehandlers(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "loadAlpnasNativeBaseHandlers" })
}

pub async fn storageheartbeattickerinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "storageHeartbeatTickerInit" })
}

pub async fn storageperformfilesystemabstractionconnectionheartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "StoragePerformFileSystemAbstractionConnectionHeartbeat" })
}

pub async fn groupstoragepoolinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "GroupStoragePoolInit" })
}

pub async fn loadstoragepoolforgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "LoadStoragePoolForGroup" })
}

pub async fn storagepoolexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "StoragePoolExists" })
}

pub async fn getallstoragepools(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "GetAllStoragePools" })
}

pub async fn getstoragepoolbyowner(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "GetStoragePoolByOwner" })
}

pub async fn getfshandlersubpathfromvpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "GetFSHandlerSubpathFromVpath" })
}

pub async fn getfshandlerbyuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "GetFsHandlerByUUID" })
}

pub async fn getallloadedfsh(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "GetAllLoadedFsh" })
}

pub async fn registerstoragesettings(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "RegisterStorageSettings" })
}

pub async fn closeallstorages(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "CloseAllStorages" })
}

pub async fn closeallstoragepools(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.go", function: "closeAllStoragePools" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
