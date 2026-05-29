//! Original Go file: `storage.pool.go`
//! Package: `main`; LOC: 678; SHA256: `01040a641e3da547aae52a68d2de8eb0403db2283fb3a4c32cd095081ac14146`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "storage.pool.go", package: "main", go_loc: 678, functions: 12, types: 0, sha256: "01040a641e3da547aae52a68d2de8eb0403db2283fb3a4c32cd095081ac14146" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/storage",
    "imuslab.com/arozos/mod/storage/bridge",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("StoragePoolEditorInit", "", 30),
    ("HandleFSHEdit", "", 61),
    ("getFSHConfigFromGroupAndUUID", "", 172),
    ("setFSHConfigByGroupAndId", "", 215),
    ("HandleFSHToggle", "", 275),
    ("HandleStoragePoolReload", "", 332),
    ("HandleStoragePoolRemove", "", 443),
    ("buildOptionFromRequestForm", "", 531),
    ("HandleListStoragePoolsConfig", "", 546),
    ("HandleListStoragePools", "", 579),
    ("HandleFSHBridging", "", 609),
    ("HandleFSHBridgeCheck", "", 657),
];

pub async fn storagepooleditorinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "StoragePoolEditorInit" })
}

pub async fn handlefshedit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleFSHEdit" })
}

pub async fn getfshconfigfromgroupanduuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "getFSHConfigFromGroupAndUUID" })
}

pub async fn setfshconfigbygroupandid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "setFSHConfigByGroupAndId" })
}

pub async fn handlefshtoggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleFSHToggle" })
}

pub async fn handlestoragepoolreload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleStoragePoolReload" })
}

pub async fn handlestoragepoolremove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleStoragePoolRemove" })
}

pub async fn buildoptionfromrequestform(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "buildOptionFromRequestForm" })
}

pub async fn handleliststoragepoolsconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleListStoragePoolsConfig" })
}

pub async fn handleliststoragepools(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleListStoragePools" })
}

pub async fn handlefshbridging(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleFSHBridging" })
}

pub async fn handlefshbridgecheck(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage.pool.go", function: "HandleFSHBridgeCheck" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
