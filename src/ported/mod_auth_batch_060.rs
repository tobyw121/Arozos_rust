//! Original Go file: `mod/auth/batch.go`
//! Package: `auth`; LOC: 151; SHA256: `eb87edd283fd2c58e6969bd51d3735e85b015961504edae1c359e847bc7ecd72`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/batch.go", package: "auth", go_loc: 151, functions: 3, types: 0, sha256: "eb87edd283fd2c58e6969bd51d3735e85b015961504edae1c359e847bc7ecd72" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleCreateUserAccountsFromCSV", "a *AuthAgent", 28),
    ("HandleUserDeleteByGroup", "a *AuthAgent", 72),
    ("ExportUserListAsCSV", "a *AuthAgent", 119),
];

pub async fn authagent_handlecreateuseraccountsfromcsv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/batch.go", function: "AuthAgent.HandleCreateUserAccountsFromCSV" })
}

pub async fn authagent_handleuserdeletebygroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/batch.go", function: "AuthAgent.HandleUserDeleteByGroup" })
}

pub async fn authagent_exportuserlistascsv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/batch.go", function: "AuthAgent.ExportUserListAsCSV" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
