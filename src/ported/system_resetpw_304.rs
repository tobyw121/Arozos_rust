//! Original Go file: `system.resetpw.go`
//! Package: `main`; LOC: 247; SHA256: `d1744a6037cb718d1df8c1f0c5c5bed0c007569eab50f90c7b609809317205f4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "system.resetpw.go", package: "main", go_loc: 247, functions: 8, types: 0, sha256: "d1744a6037cb718d1df8c1f0c5c5bed0c007569eab50f90c7b609809317205f4" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "html",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("system_resetpw_init", "", 24),
    ("system_resetpw_validateResetKeyHandler", "", 31),
    ("system_resetpw_confirmReset", "", 59),
    ("system_resetpw_validateResetKey", "", 93),
    ("system_resetpw_handlePasswordReset", "", 110),
    ("system_resetpw_serveIdEnterInterface", "", 150),
    ("system_resetpw_alpnasResetSystem", "", 173),
    ("system_resetpw_renderAlpnasResetForm", "", 234),
];

pub async fn system_resetpw_init(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_init" })
}

pub async fn system_resetpw_validateresetkeyhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_validateResetKeyHandler" })
}

pub async fn system_resetpw_confirmreset(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_confirmReset" })
}

pub async fn system_resetpw_validateresetkey(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_validateResetKey" })
}

pub async fn system_resetpw_handlepasswordreset(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_handlePasswordReset" })
}

pub async fn system_resetpw_serveidenterinterface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_serveIdEnterInterface" })
}

pub async fn system_resetpw_alpnasresetsystem(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_alpnasResetSystem" })
}

pub async fn system_resetpw_renderalpnasresetform(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.resetpw.go", function: "system_resetpw_renderAlpnasResetForm" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
