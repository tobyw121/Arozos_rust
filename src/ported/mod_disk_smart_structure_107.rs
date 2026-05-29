//! Original Go file: `mod/disk/smart/structure.go`
//! Package: `smart`; LOC: 220; SHA256: `a826b7d496e4dd6e1937e941b36c01284b52bc9aae8b7930d4a7980c92d03ef4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/smart/structure.go", package: "smart", go_loc: 220, functions: 0, types: 2, sha256: "a826b7d496e4dd6e1937e941b36c01284b52bc9aae8b7930d4a7980c92d03ef4" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("DevicesList", "struct", 4),
    ("DeviceSMART", "struct", 31),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[];

pub fn migration_status() -> LegacyModuleStatus { STATUS }
