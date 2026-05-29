//! Original Go file: `mod/fileservers/servers/samba/sambashare.go`
//! Package: `samba`; LOC: 51; SHA256: `2c56ce5b7c752b9c2fb94240510f5f5395f3844fb6b149c7d35025fc7500f09e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/samba/sambashare.go", package: "samba", go_loc: 51, functions: 3, types: 0, sha256: "2c56ce5b7c752b9c2fb94240510f5f5395f3844fb6b149c7d35025fc7500f09e" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SaveToConfig", "s *ShareConfig", 11),
    ("Remove", "s *ShareConfig", 31),
    ("Rename", "s *ShareConfig", 35),
];

pub async fn shareconfig_savetoconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/sambashare.go", function: "ShareConfig.SaveToConfig" })
}

pub async fn shareconfig_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/sambashare.go", function: "ShareConfig.Remove" })
}

pub async fn shareconfig_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/sambashare.go", function: "ShareConfig.Rename" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
