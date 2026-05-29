//! Original Go file: `mod/share/shareEntry/utils.go`
//! Package: `shareEntry`; LOC: 10; SHA256: `debdb89c44f508601d73f049530974860f250a0639c5a878031e75533f7d0661`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/share/shareEntry/utils.go", package: "shareEntry", go_loc: 10, functions: 1, types: 0, sha256: "debdb89c44f508601d73f049530974860f250a0639c5a878031e75533f7d0661" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("stringInSlice", "", 3),
];

pub async fn stringinslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/utils.go", function: "stringInSlice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
