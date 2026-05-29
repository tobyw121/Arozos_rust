//! Original Go file: `mod/auth/ldap/web_login.go`
//! Package: `ldap`; LOC: 226; SHA256: `610b7fa4c913ad14d08b6b26440b6fb6ccec6b4c244fcfc70b2ef31be7e35cfc`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/ldap/web_login.go", package: "ldap", go_loc: 226, functions: 5, types: 0, sha256: "610b7fa4c913ad14d08b6b26440b6fb6ccec6b4c244fcfc70b2ef31be7e35cfc" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleLoginPage", "ldap *ldapHandler", 14),
    ("HandleNewPasswordPage", "ldap *ldapHandler", 42),
    ("HandleLogin", "ldap *ldapHandler", 83),
    ("HandleSetPassword", "ldap *ldapHandler", 151),
    ("HandleCheckLDAP", "ldap *ldapHandler", 211),
];

pub async fn ldaphandler_handleloginpage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_login.go", function: "ldapHandler.HandleLoginPage" })
}

pub async fn ldaphandler_handlenewpasswordpage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_login.go", function: "ldapHandler.HandleNewPasswordPage" })
}

pub async fn ldaphandler_handlelogin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_login.go", function: "ldapHandler.HandleLogin" })
}

pub async fn ldaphandler_handlesetpassword(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_login.go", function: "ldapHandler.HandleSetPassword" })
}

pub async fn ldaphandler_handlecheckldap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/web_login.go", function: "ldapHandler.HandleCheckLDAP" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
