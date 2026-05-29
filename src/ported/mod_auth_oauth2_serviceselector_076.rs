//! Original Go file: `mod/auth/oauth2/serviceSelector.go`
//! Package: `oauth2`; LOC: 53; SHA256: `3c5ab00d9a44b06d565e19456e4daa14e6b3d229587637116eab5fb791043251`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/serviceSelector.go", package: "oauth2", go_loc: 53, functions: 3, types: 0, sha256: "3c5ab00d9a44b06d565e19456e4daa14e6b3d229587637116eab5fb791043251" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "golang.org/x/oauth2",
    "imuslab.com/arozos/mod/database",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getScope", "", 11),
    ("getEndpoint", "", 26),
    ("getUserInfo", "", 41),
];

pub async fn getscope(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/serviceSelector.go", function: "getScope" })
}

pub async fn getendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/serviceSelector.go", function: "getEndpoint" })
}

pub async fn getuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/serviceSelector.go", function: "getUserInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
