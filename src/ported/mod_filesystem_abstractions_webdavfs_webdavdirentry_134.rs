//! Original Go file: `mod/filesystem/abstractions/webdavfs/webdavDirEntry.go`
//! Package: `webdavfs`; LOC: 29; SHA256: `b1bfc2af8d97aa6088bec6e2a8a2c377dc25819abe839baddd57d3228f65fa06`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/webdavfs/webdavDirEntry.go", package: "webdavfs", go_loc: 29, functions: 5, types: 1, sha256: "b1bfc2af8d97aa6088bec6e2a8a2c377dc25819abe839baddd57d3228f65fa06" };

pub const GO_IMPORTS: &[&str] = &[
    "io/fs",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("WebdavDirEntry", "struct", 5),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("newDirEntryFromFileInfo", "", 9),
    ("Name", "de WebdavDirEntry", 15),
    ("IsDir", "de WebdavDirEntry", 19),
    ("Type", "de WebdavDirEntry", 23),
    ("Info", "de WebdavDirEntry", 27),
];

pub async fn newdirentryfromfileinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavDirEntry.go", function: "newDirEntryFromFileInfo" })
}

pub async fn webdavdirentry_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavDirEntry.go", function: "WebdavDirEntry.Name" })
}

pub async fn webdavdirentry_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavDirEntry.go", function: "WebdavDirEntry.IsDir" })
}

pub async fn webdavdirentry_type(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavDirEntry.go", function: "WebdavDirEntry.Type" })
}

pub async fn webdavdirentry_info(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavDirEntry.go", function: "WebdavDirEntry.Info" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
