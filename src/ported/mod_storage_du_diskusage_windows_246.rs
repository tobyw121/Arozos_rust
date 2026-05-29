//! Original Go file: `mod/storage/du/diskusage_windows.go`
//! Package: `du`; LOC: 55; SHA256: `a7299fe19e5b3c350e361e089d3dc6abcf0c95d72ad1e337de38a82e3659065d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/du/diskusage_windows.go", package: "du", go_loc: 55, functions: 6, types: 1, sha256: "a7299fe19e5b3c350e361e089d3dc6abcf0c95d72ad1e337de38a82e3659065d" };

pub const GO_IMPORTS: &[&str] = &[
    "syscall",
    "unsafe",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("DiskUsage", "struct", 8),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDiskUsage", "", 16),
    ("Free", "du *DiskUsage", 33),
    ("Available", "du *DiskUsage", 38),
    ("Size", "du *DiskUsage", 43),
    ("Used", "du *DiskUsage", 48),
    ("Usage", "du *DiskUsage", 53),
];

pub async fn newdiskusage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage_windows.go", function: "NewDiskUsage" })
}

pub async fn diskusage_free(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage_windows.go", function: "DiskUsage.Free" })
}

pub async fn diskusage_available(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage_windows.go", function: "DiskUsage.Available" })
}

pub async fn diskusage_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage_windows.go", function: "DiskUsage.Size" })
}

pub async fn diskusage_used(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage_windows.go", function: "DiskUsage.Used" })
}

pub async fn diskusage_usage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage_windows.go", function: "DiskUsage.Usage" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
