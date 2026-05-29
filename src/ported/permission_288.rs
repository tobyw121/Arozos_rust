//! Original Go file: `permission.go`
//! Package: `main`; LOC: 70; SHA256: `911303dc922fa40e5181a65fb1f90d895d7ee71c5f548824214ea2f94dfb7ae7`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "permission.go", package: "main", go_loc: 70, functions: 2, types: 0, sha256: "911303dc922fa40e5181a65fb1f90d895d7ee71c5f548824214ea2f94dfb7ae7" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("permissionNewHandler", "", 12),
    ("permissionInit", "", 23),
];

pub async fn permissionnewhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "permission.go", function: "permissionNewHandler" })
}

pub async fn permissioninit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "permission.go", function: "permissionInit" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
