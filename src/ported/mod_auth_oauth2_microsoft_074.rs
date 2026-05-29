//! Original Go file: `mod/auth/oauth2/microsoft.go`
//! Package: `oauth2`; LOC: 48; SHA256: `30cdc4895cfda2246009ff2a37206071a7a998d03cda2e590cb27e5ca61a0b48`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/microsoft.go", package: "oauth2", go_loc: 48, functions: 3, types: 1, sha256: "30cdc4895cfda2246009ff2a37206071a7a998d03cda2e590cb27e5ca61a0b48" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "golang.org/x/oauth2",
    "io",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("MicrosoftField", "struct", 11),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("microsoftScope", "", 20),
    ("microsoftEndpoint", "", 24),
    ("microsoftUserInfo", "", 31),
];

pub async fn microsoftscope(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/microsoft.go", function: "microsoftScope" })
}

pub async fn microsoftendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/microsoft.go", function: "microsoftEndpoint" })
}

pub async fn microsoftuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/microsoft.go", function: "microsoftUserInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
