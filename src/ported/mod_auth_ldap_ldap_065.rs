//! Original Go file: `mod/auth/ldap/ldap.go`
//! Package: `ldap`; LOC: 173; SHA256: `5b01207c3c1adc94c8d1015e51b4ec5f300f52e303a5e2ab8b67bc745e626852`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/ldap/ldap.go", package: "ldap", go_loc: 173, functions: 5, types: 4, sha256: "5b01207c3c1adc94c8d1015e51b4ec5f300f52e303a5e2ab8b67bc745e626852" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/go-ldap/ldap",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/auth/ldap/ldapreader",
    "imuslab.com/arozos/mod/auth/oauth2/syncdb",
    "imuslab.com/arozos/mod/auth/register",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/time/nightly",
    "imuslab.com/arozos/mod/user",
    "log",
    "regexp",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ldapHandler", "struct", 18),
    ("Config", "struct", 30),
    ("UserAccount", "struct", 38),
    ("syncorizeUserReturnInterface", "struct", 45),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewLdapHandler", "", 53),
    ("getAllUser", "ldap *ldapHandler", 86),
    ("convertGroup", "ldap *ldapHandler", 109),
    ("NightlySync", "ldap *ldapHandler", 141),
    ("SynchronizeUserFromLDAP", "ldap *ldapHandler", 151),
];

pub async fn newldaphandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldap.go", function: "NewLdapHandler" })
}

pub async fn ldaphandler_getalluser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldap.go", function: "ldapHandler.getAllUser" })
}

pub async fn ldaphandler_convertgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldap.go", function: "ldapHandler.convertGroup" })
}

pub async fn ldaphandler_nightlysync(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldap.go", function: "ldapHandler.NightlySync" })
}

pub async fn ldaphandler_synchronizeuserfromldap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldap.go", function: "ldapHandler.SynchronizeUserFromLDAP" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
