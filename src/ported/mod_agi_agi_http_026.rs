//! Original Go file: `mod/agi/agi.http.go`
//! Package: `agi`; LOC: 293; SHA256: `2720c3db4e57480dd57e4324084811769083ec659c408658ab6c09773e861eda`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.http.go", package: "agi", go_loc: 293, functions: 2, types: 0, sha256: "2720c3db4e57480dd57e4324084811769083ec659c408658ab6c09773e861eda" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "encoding/base64",
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "io",
    "log",
    "net/http",
    "net/url",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HTTPLibRegister", "g *Gateway", 27),
    ("injectHTTPFunctions", "g *Gateway", 34),
];

pub async fn gateway_httplibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.http.go", function: "Gateway.HTTPLibRegister" })
}

pub async fn gateway_injecthttpfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.http.go", function: "Gateway.injectHTTPFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
