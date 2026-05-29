//! Original Go file: `mod/agi/agi.system.go`
//! Package: `agi`; LOC: 358; SHA256: `6ff0aff8743ee59347bbd3f7dfd18ccfb9d708969433c97cbe13eeb65f994e5b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.system.go", package: "agi", go_loc: 358, functions: 1, types: 0, sha256: "6ff0aff8743ee59347bbd3f7dfd18ccfb9d708969433c97cbe13eeb65f994e5b" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("injectStandardLibs", "g *Gateway", 18),
];

pub async fn gateway_injectstandardlibs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.system.go", function: "Gateway.injectStandardLibs" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
