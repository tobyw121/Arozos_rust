//! Original Go file: `mod/auth/oauth2/oauth2.go`
//! Package: `oauth2`; LOC: 342; SHA256: `4e72c43a82ad7958a5909caee329dccd087d7b0fc6f46123fc73b200da480e80`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/oauth2.go", package: "oauth2", go_loc: 342, functions: 9, types: 2, sha256: "4e72c43a82ad7958a5909caee329dccd087d7b0fc6f46123fc73b200da480e80" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "encoding/json",
    "golang.org/x/oauth2",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/auth/oauth2/syncdb",
    "imuslab.com/arozos/mod/auth/register",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("OauthHandler", "struct", 20),
    ("Config", "struct", 28),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewOauthHandler", "", 39),
    ("HandleLogin", "oh *OauthHandler", 64),
    ("HandleAuthorize", "oh *OauthHandler", 87),
    ("CheckOAuth", "oh *OauthHandler", 172),
    ("addCookie", "oh *OauthHandler", 197),
    ("ReadConfig", "oh *OauthHandler", 207),
    ("WriteConfig", "oh *OauthHandler", 245),
    ("readSingleConfig", "oh *OauthHandler", 326),
    ("readSingleConfig", "", 335),
];

pub async fn newoauthhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "NewOauthHandler" })
}

pub async fn oauthhandler_handlelogin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.HandleLogin" })
}

pub async fn oauthhandler_handleauthorize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.HandleAuthorize" })
}

pub async fn oauthhandler_checkoauth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.CheckOAuth" })
}

pub async fn oauthhandler_addcookie(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.addCookie" })
}

pub async fn oauthhandler_readconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.ReadConfig" })
}

pub async fn oauthhandler_writeconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.WriteConfig" })
}

pub async fn oauthhandler_readsingleconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "OauthHandler.readSingleConfig" })
}

pub async fn readsingleconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/oauth2.go", function: "readSingleConfig" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
