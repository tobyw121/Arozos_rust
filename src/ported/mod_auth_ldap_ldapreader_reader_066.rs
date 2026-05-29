//! Original Go file: `mod/auth/ldap/ldapreader/reader.go`
//! Package: `ldapreader`; LOC: 97; SHA256: `d241b82ffee38fea3c6f7c49188d425b051c92f76af835f7d3ad0514d4895bb0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/ldap/ldapreader/reader.go", package: "ldapreader", go_loc: 97, functions: 5, types: 1, sha256: "d241b82ffee38fea3c6f7c49188d425b051c92f76af835f7d3ad0514d4895bb0" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "github.com/go-ldap/ldap",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("LdapReader", "struct", 10),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewLDAPReader", "", 18),
    ("GetUser", "handler *LdapReader", 30),
    ("GetAllUser", "handler *LdapReader", 41),
    ("Authenticate", "handler *LdapReader", 45),
    ("retrieveInformation", "handler *LdapReader", 67),
];

pub async fn newldapreader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldapreader/reader.go", function: "NewLDAPReader" })
}

pub async fn ldapreader_getuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldapreader/reader.go", function: "LdapReader.GetUser" })
}

pub async fn ldapreader_getalluser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldapreader/reader.go", function: "LdapReader.GetAllUser" })
}

pub async fn ldapreader_authenticate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldapreader/reader.go", function: "LdapReader.Authenticate" })
}

pub async fn ldapreader_retrieveinformation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/ldap/ldapreader/reader.go", function: "LdapReader.retrieveInformation" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
