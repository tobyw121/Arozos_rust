//! Original Go file: `mod/auth/internal.go`
//! Package: `auth`; LOC: 63; SHA256: `e524d512d9577d9c01defb31874f11fafc24005d08ad57b3893ff73380dd77db`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/internal.go", package: "auth", go_loc: 63, functions: 6, types: 0, sha256: "e524d512d9577d9c01defb31874f11fafc24005d08ad57b3893ff73380dd77db" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("sendTextResponse", "", 9),
    ("sendJSONResponse", "", 14),
    ("sendErrorResponse", "", 19),
    ("sendOK", "", 24),
    ("Mv", "", 30),
    ("inSlice", "", 56),
];

pub async fn sendtextresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/internal.go", function: "sendTextResponse" })
}

pub async fn sendjsonresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/internal.go", function: "sendJSONResponse" })
}

pub async fn senderrorresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/internal.go", function: "sendErrorResponse" })
}

pub async fn sendok(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/internal.go", function: "sendOK" })
}

pub async fn mv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/internal.go", function: "Mv" })
}

pub async fn inslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/internal.go", function: "inSlice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
