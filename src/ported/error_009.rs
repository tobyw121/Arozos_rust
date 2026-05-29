//! Original Go file: `error.go`
//! Package: `main`; LOC: 97; SHA256: `38a1e1b60abd88f5bc88e833c3a34f7db169f50fcee6f57a15dd337ff7cfd551`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "error.go", package: "main", go_loc: 97, functions: 4, types: 0, sha256: "38a1e1b60abd88f5bc88e833c3a34f7db169f50fcee6f57a15dd337ff7cfd551" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem",
    "net/http",
    "os",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("errorHandleNotFound", "", 16),
    ("errorHandleInternalServerError", "", 39),
    ("errorHandlePermissionDenied", "", 62),
    ("getRootEscapeFromCurrentPath", "", 83),
];

pub async fn errorhandlenotfound(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "error.go", function: "errorHandleNotFound" })
}

pub async fn errorhandleinternalservererror(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "error.go", function: "errorHandleInternalServerError" })
}

pub async fn errorhandlepermissiondenied(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "error.go", function: "errorHandlePermissionDenied" })
}

pub async fn getrootescapefromcurrentpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "error.go", function: "getRootEscapeFromCurrentPath" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
