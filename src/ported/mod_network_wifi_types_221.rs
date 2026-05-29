//! Original Go file: `mod/network/wifi/types.go`
//! Package: `wifi`; LOC: 48; SHA256: `f4f54a504043e49011724a2b372aab05106f772aaf9a7c0706a952f142fa12ab`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/wifi/types.go", package: "wifi", go_loc: 48, functions: 0, types: 2, sha256: "f4f54a504043e49011724a2b372aab05106f772aaf9a7c0706a952f142fa12ab" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("WiFiInfo", "struct", 3),
    ("WiFiConnectionResult", "struct", 14),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[];

pub fn migration_status() -> LegacyModuleStatus { STATUS }
