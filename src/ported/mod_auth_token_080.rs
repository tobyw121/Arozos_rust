//! Original Go file: `mod/auth/token.go`
//! Package: `auth`; LOC: 91; SHA256: `f81e5a99c4e2eac67a14991e8d3f158b6c91d3100324ee2f729e6e4a328ac08b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/token.go", package: "auth", go_loc: 91, functions: 5, types: 1, sha256: "f81e5a99c4e2eac67a14991e8d3f158b6c91d3100324ee2f729e6e4a328ac08b" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/satori/go.uuid",
    "net/http",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("token", "struct", 16),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewTokenFromRequest", "a *AuthAgent", 22),
    ("NewToken", "a *AuthAgent", 36),
    ("GetTokenOwner", "a *AuthAgent", 51),
    ("TokenValid", "a *AuthAgent", 60),
    ("ClearTokenStore", "a *AuthAgent", 83),
];

pub async fn authagent_newtokenfromrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/token.go", function: "AuthAgent.NewTokenFromRequest" })
}

pub async fn authagent_newtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/token.go", function: "AuthAgent.NewToken" })
}

pub async fn authagent_gettokenowner(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/token.go", function: "AuthAgent.GetTokenOwner" })
}

pub async fn authagent_tokenvalid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/token.go", function: "AuthAgent.TokenValid" })
}

pub async fn authagent_cleartokenstore(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/token.go", function: "AuthAgent.ClearTokenStore" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
