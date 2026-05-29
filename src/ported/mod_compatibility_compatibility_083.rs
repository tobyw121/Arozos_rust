//! Original Go file: `mod/compatibility/compatibility.go`
//! Package: `compatibility`; LOC: 10; SHA256: `12a5e1230193e3be3b2c0497a37441be09684ba62e846673f2ee69be1aca1354`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/compatibility/compatibility.go", package: "compatibility", go_loc: 10, functions: 0, types: 0, sha256: "12a5e1230193e3be3b2c0497a37441be09684ba62e846673f2ee69be1aca1354" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[];

pub fn migration_status() -> LegacyModuleStatus { STATUS }
