//! Original Go file: `mod/agi/agi.user.go`
//! Package: `agi`; LOC: 286; SHA256: `6bc708acf4066df91a74f46ea13e8300e2e1c324ce85393f719f88cef9bed371`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.user.go", package: "agi", go_loc: 286, functions: 1, types: 0, sha256: "6bc708acf4066df91a74f46ea13e8300e2e1c324ce85393f719f88cef9bed371" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/user",
    "log",
    "net/http",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("injectUserFunctions", "g *Gateway", 21),
];

pub async fn gateway_injectuserfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.user.go", function: "Gateway.injectUserFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
