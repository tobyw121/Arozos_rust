//! Original Go file: `mod/share/shareEntry/shareOptions.go`
//! Package: `shareEntry`; LOC: 27; SHA256: `3b233e5e3b1ab226645ec1031e352293596ffeec5ad7d85ecc2e296eeba7d954`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/share/shareEntry/shareOptions.go", package: "shareEntry", go_loc: 27, functions: 2, types: 0, sha256: "3b233e5e3b1ab226645ec1031e352293596ffeec5ad7d85ecc2e296eeba7d954" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("IsOwnedBy", "s *ShareOption", 3),
    ("IsAccessibleBy", "s *ShareOption", 7),
];

pub async fn shareoption_isownedby(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareOptions.go", function: "ShareOption.IsOwnedBy" })
}

pub async fn shareoption_isaccessibleby(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareOptions.go", function: "ShareOption.IsAccessibleBy" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
