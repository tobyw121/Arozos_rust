//! Original Go file: `mod/fileservers/typedef.go`
//! Package: `fileservers`; LOC: 30; SHA256: `eb7a533780604931c57b815f381f782e3d50ca4286581cd2fd00fdd18d4a4849`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/typedef.go", package: "fileservers", go_loc: 30, functions: 0, types: 2, sha256: "eb7a533780604931c57b815f381f782e3d50ca4286581cd2fd00fdd18d4a4849" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Endpoint", "struct", 9),
    ("Server", "struct", 15),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[];

pub fn migration_status() -> LegacyModuleStatus { STATUS }
