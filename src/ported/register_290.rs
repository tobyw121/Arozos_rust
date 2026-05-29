//! Original Go file: `register.go`
//! Package: `main`; LOC: 158; SHA256: `40f1a854316265bf936f5dc3ae1576637222317fec67795e8fc9b9acb91fb57b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "register.go", package: "main", go_loc: 158, functions: 6, types: 0, sha256: "40f1a854316265bf936f5dc3ae1576637222317fec67795e8fc9b9acb91fb57b" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "fmt",
    "imuslab.com/arozos/mod/auth/register",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("RegisterSystemInit", "", 19),
    ("register_handleRegisterCleaning", "", 92),
    ("register_handleEmailListing", "", 98),
    ("register_handleSetDefaultGroup", "", 126),
    ("register_handleGetAllowRegistry", "", 146),
    ("register_handleToggleRegistry", "", 151),
];

pub async fn registersysteminit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "register.go", function: "RegisterSystemInit" })
}

pub async fn register_handleregistercleaning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "register.go", function: "register_handleRegisterCleaning" })
}

pub async fn register_handleemaillisting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "register.go", function: "register_handleEmailListing" })
}

pub async fn register_handlesetdefaultgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "register.go", function: "register_handleSetDefaultGroup" })
}

pub async fn register_handlegetallowregistry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "register.go", function: "register_handleGetAllowRegistry" })
}

pub async fn register_handletoggleregistry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "register.go", function: "register_handleToggleRegistry" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
