//! Original Go file: `mod/auth/ldap/web_admin.go`
//! Package: `ldap`; LOC: 180; SHA256: `bc8fbf43679425fd54684e85d184b920dd9b3d561573f0aae0112a9993fef244`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/ldap/web_admin.go", package: "ldap", go_loc: 180, functions: 5, types: 0, sha256: "bc8fbf43679425fd54684e85d184b920dd9b3d561573f0aae0112a9993fef244" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/auth/ldap/ldapreader",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "regexp",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ReadConfig", "ldap *ldapHandler", 15),
    ("WriteConfig", "ldap *ldapHandler", 46),
    ("TestConnection", "ldap *ldapHandler", 104),
    ("checkCurrUserAdmin", "ldap *ldapHandler", 130),
    ("SynchronizeUser", "ldap *ldapHandler", 159),
];

pub async fn ldaphandler_readconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_admin.go", function: "ldapHandler.ReadConfig" })
}

pub async fn ldaphandler_writeconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_admin.go", function: "ldapHandler.WriteConfig" })
}

pub async fn ldaphandler_testconnection(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_admin.go", function: "ldapHandler.TestConnection" })
}

pub async fn ldaphandler_checkcurruseradmin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_admin.go", function: "ldapHandler.checkCurrUserAdmin" })
}

pub async fn ldaphandler_synchronizeuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_admin.go", function: "ldapHandler.SynchronizeUser" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
