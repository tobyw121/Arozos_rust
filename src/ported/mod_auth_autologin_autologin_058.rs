//! Original Go file: `mod/auth/autologin/autologin.go`
//! Package: `autologin`; LOC: 77; SHA256: `7c3e6f115a56b80d70df882d8c3d855241a151d4b3c6613bc82d9cfa28b64461`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/autologin/autologin.go", package: "autologin", go_loc: 77, functions: 4, types: 1, sha256: "7c3e6f115a56b80d70df882d8c3d855241a151d4b3c6613bc82d9cfa28b64461" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("AutoLoginHandler", "struct", 11),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewAutoLoginHandler", "", 15),
    ("HandleUserTokensListing", "a *AutoLoginHandler", 22),
    ("HandleUserTokenCreation", "a *AutoLoginHandler", 44),
    ("HandleUserTokenRemoval", "a *AutoLoginHandler", 65),
];

pub async fn newautologinhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/autologin/autologin.go", function: "NewAutoLoginHandler" })
}

pub async fn autologinhandler_handleusertokenslisting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/autologin/autologin.go", function: "AutoLoginHandler.HandleUserTokensListing" })
}

pub async fn autologinhandler_handleusertokencreation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/autologin/autologin.go", function: "AutoLoginHandler.HandleUserTokenCreation" })
}

pub async fn autologinhandler_handleusertokenremoval(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/autologin/autologin.go", function: "AutoLoginHandler.HandleUserTokenRemoval" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
