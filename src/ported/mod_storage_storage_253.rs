//! Original Go file: `mod/storage/storage.go`
//! Package: `storage`; LOC: 137; SHA256: `fbcd1577e1301e45f992f893d8629b4ea807522de96512bc385669c6cece6cdc`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/storage.go", package: "storage", go_loc: 137, functions: 9, types: 1, sha256: "fbcd1577e1301e45f992f893d8629b4ea807522de96512bc385669c6cece6cdc" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "os",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("StoragePool", "struct", 21),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("init", "", 36),
    ("NewStoragePool", "", 41),
    ("ContainDiskID", "s *StoragePool", 57),
    ("HasHigherOrEqualPermissionThan", "s *StoragePool", 68),
    ("GetFSHandlerFromVirtualPath", "s *StoragePool", 78),
    ("GetFsHandlerByUUID", "s *StoragePool", 92),
    ("AttachFsHandler", "s *StoragePool", 108),
    ("DetachFsHandler", "s *StoragePool", 118),
    ("Close", "s *StoragePool", 130),
];

pub async fn init(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "init" })
}

pub async fn newstoragepool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "NewStoragePool" })
}

pub async fn storagepool_containdiskid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.ContainDiskID" })
}

pub async fn storagepool_hashigherorequalpermissionthan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.HasHigherOrEqualPermissionThan" })
}

pub async fn storagepool_getfshandlerfromvirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.GetFSHandlerFromVirtualPath" })
}

pub async fn storagepool_getfshandlerbyuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.GetFsHandlerByUUID" })
}

pub async fn storagepool_attachfshandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.AttachFsHandler" })
}

pub async fn storagepool_detachfshandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.DetachFsHandler" })
}

pub async fn storagepool_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/storage.go", function: "StoragePool.Close" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
