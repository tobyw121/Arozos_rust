//! Original Go file: `mod/auth/auth.go`
//! Package: `auth`; LOC: 642; SHA256: `bc6b9f9f7762c2030f2fc069f7570b52eb7ae356b3a7978b3929ea1b38e09686`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/auth.go", package: "auth", go_loc: 642, functions: 27, types: 2, sha256: "bc6b9f9f7762c2030f2fc069f7570b52eb7ae356b3a7978b3929ea1b38e09686" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/sha512",
    "encoding/hex",
    "errors",
    "github.com/gorilla/sessions",
    "imuslab.com/arozos/mod/auth/accesscontrol/blacklist",
    "imuslab.com/arozos/mod/auth/accesscontrol/whitelist",
    "imuslab.com/arozos/mod/auth/authlogger",
    "imuslab.com/arozos/mod/auth/explogin",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/network",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "strconv",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("AuthAgent", "struct", 45),
    ("AuthEndpoints", "struct", 76),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("normalizeUsername", "", 84),
    ("validateUsername", "", 88),
    ("requestUsesSecureTransport", "", 101),
    ("buildSessionOptions", "", 111),
    ("NewAuthenticationAgent", "", 122),
    ("Close", "a *AuthAgent", 194),
    ("HandleCheckAuth", "a *AuthAgent", 203),
    ("HandleLogin", "a *AuthAgent", 214),
    ("ValidateUsernameAndPassword", "a *AuthAgent", 299),
    ("ValidateUsernameAndPasswordWithReason", "a *AuthAgent", 305),
    ("ValidateLoginRequest", "a *AuthAgent", 327),
    ("ValidateLoginIpAccess", "a *AuthAgent", 337),
    ("LoginUserByRequest", "a *AuthAgent", 354),
    ("HandleLogout", "a *AuthAgent", 371),
    ("Logout", "a *AuthAgent", 397),
    ("GetUserName", "a *AuthAgent", 411),
    ("CheckLogin", "a *AuthAgent", 423),
    ("HandleRegister", "a *AuthAgent", 432),
    ("CheckAuth", "a *AuthAgent", 481),
    ("HandleUnregister", "a *AuthAgent", 492),
    ("UnregisterUser", "a *AuthAgent", 527),
    ("GetUserCounts", "a *AuthAgent", 549),
    ("ListUsers", "a *AuthAgent", 566),
    ("UserExists", "a *AuthAgent", 579),
    ("UpdateSessionExpireTime", "a *AuthAgent", 596),
    ("CreateUserAccount", "a *AuthAgent", 616),
    ("Hash", "", 638),
];

pub async fn normalizeusername(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "normalizeUsername" })
}

pub async fn validateusername(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "validateUsername" })
}

pub async fn requestusessecuretransport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "requestUsesSecureTransport" })
}

pub async fn buildsessionoptions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "buildSessionOptions" })
}

pub async fn newauthenticationagent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "NewAuthenticationAgent" })
}

pub async fn authagent_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.Close" })
}

pub async fn authagent_handlecheckauth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.HandleCheckAuth" })
}

pub async fn authagent_handlelogin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.HandleLogin" })
}

pub async fn authagent_validateusernameandpassword(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.ValidateUsernameAndPassword" })
}

pub async fn authagent_validateusernameandpasswordwithreason(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.ValidateUsernameAndPasswordWithReason" })
}

pub async fn authagent_validateloginrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.ValidateLoginRequest" })
}

pub async fn authagent_validateloginipaccess(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.ValidateLoginIpAccess" })
}

pub async fn authagent_loginuserbyrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.LoginUserByRequest" })
}

pub async fn authagent_handlelogout(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.HandleLogout" })
}

pub async fn authagent_logout(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.Logout" })
}

pub async fn authagent_getusername(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.GetUserName" })
}

pub async fn authagent_checklogin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.CheckLogin" })
}

pub async fn authagent_handleregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.HandleRegister" })
}

pub async fn authagent_checkauth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.CheckAuth" })
}

pub async fn authagent_handleunregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.HandleUnregister" })
}

pub async fn authagent_unregisteruser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.UnregisterUser" })
}

pub async fn authagent_getusercounts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.GetUserCounts" })
}

pub async fn authagent_listusers(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.ListUsers" })
}

pub async fn authagent_userexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.UserExists" })
}

pub async fn authagent_updatesessionexpiretime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.UpdateSessionExpireTime" })
}

pub async fn authagent_createuseraccount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "AuthAgent.CreateUserAccount" })
}

pub async fn hash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/auth.go", function: "Hash" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
