//! Original Go file: `mod/storage/static.go`
//! Package: `storage`; LOC: 16; SHA256: `a4f89c6a0c0a39931c46e46ba0846cbf8fb2878d46545474a4ff7806ee369244`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/static.go", package: "storage", go_loc: 16, functions: 1, types: 0, sha256: "a4f89c6a0c0a39931c46e46ba0846cbf8fb2878d46545474a4ff7806ee369244" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetDriveCapacity", "", 10),
];

pub async fn getdrivecapacity(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/static.go", function: "GetDriveCapacity" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
