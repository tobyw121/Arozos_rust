//! Original Go file: `mod/disk/raid/raiddetails.go`
//! Package: `raid`; LOC: 176; SHA256: `1fbc43fe99b0733631b338ef7ced4aa38e45425af107d6ce16bc8424053f313e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/raiddetails.go", package: "raid", go_loc: 176, functions: 3, types: 2, sha256: "1fbc43fe99b0733631b338ef7ced4aa38e45425af107d6ce16bc8424053f313e" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "os/exec",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("RAIDInfo", "struct", 12),
    ("DeviceInfo", "struct", 37),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetRAIDInfo", "m *Manager", 45),
    ("parseRAIDInfo", "", 61),
    ("PrettyPrintRAIDInfo", "info *RAIDInfo", 150),
];

pub async fn manager_getraidinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raiddetails.go", function: "Manager.GetRAIDInfo" })
}

pub async fn parseraidinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raiddetails.go", function: "parseRAIDInfo" })
}

pub async fn raidinfo_prettyprintraidinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raiddetails.go", function: "RAIDInfo.PrettyPrintRAIDInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
