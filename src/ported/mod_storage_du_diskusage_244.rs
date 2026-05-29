//! Original Go file: `mod/storage/du/diskusage.go`
//! Package: `du`; LOC: 44; SHA256: `df3e8875ad939ffad6a0775381071e1f31b915455bc9ceb9a53387dc357e81ad`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/du/diskusage.go", package: "du", go_loc: 44, functions: 6, types: 1, sha256: "df3e8875ad939ffad6a0775381071e1f31b915455bc9ceb9a53387dc357e81ad" };

pub const GO_IMPORTS: &[&str] = &[
    "syscall",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("DiskUsage", "struct", 8),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDiskUsage", "", 14),
    ("Free", "du *DiskUsage", 22),
    ("Available", "du *DiskUsage", 27),
    ("Size", "du *DiskUsage", 32),
    ("Used", "du *DiskUsage", 37),
    ("Usage", "du *DiskUsage", 42),
];

pub async fn newdiskusage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage.go", function: "NewDiskUsage" })
}

pub async fn diskusage_free(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage.go", function: "DiskUsage.Free" })
}

pub async fn diskusage_available(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage.go", function: "DiskUsage.Available" })
}

pub async fn diskusage_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage.go", function: "DiskUsage.Size" })
}

pub async fn diskusage_used(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage.go", function: "DiskUsage.Used" })
}

pub async fn diskusage_usage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/du/diskusage.go", function: "DiskUsage.Usage" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
