//! Original Go file: `mod/auth/oauth2/google.go`
//! Package: `oauth2`; LOC: 43; SHA256: `95a69da2e3ffe98c1719d11529597e02ff407c6a4456b79b5ce27c61d3418668`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/google.go", package: "oauth2", go_loc: 43, functions: 3, types: 1, sha256: "95a69da2e3ffe98c1719d11529597e02ff407c6a4456b79b5ce27c61d3418668" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "golang.org/x/oauth2",
    "golang.org/x/oauth2/google",
    "io",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("GoogleField", "struct", 12),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("googleScope", "", 23),
    ("googleEndpoint", "", 28),
    ("googleUserInfo", "", 32),
];

pub async fn googlescope(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/google.go", function: "googleScope" })
}

pub async fn googleendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/google.go", function: "googleEndpoint" })
}

pub async fn googleuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/google.go", function: "googleUserInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
