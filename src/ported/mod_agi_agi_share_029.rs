//! Original Go file: `mod/agi/agi.share.go`
//! Package: `agi`; LOC: 137; SHA256: `49613ff02e14dbf20829557a18b7d9fb8d1d43c42e54992c329d3e671f45fa76`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.share.go", package: "agi", go_loc: 137, functions: 2, types: 0, sha256: "49613ff02e14dbf20829557a18b7d9fb8d1d43c42e54992c329d3e671f45fa76" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "log",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ShareLibRegister", "g *Gateway", 11),
    ("injectShareFunctions", "g *Gateway", 18),
];

pub async fn gateway_sharelibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.share.go", function: "Gateway.ShareLibRegister" })
}

pub async fn gateway_injectsharefunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.share.go", function: "Gateway.injectShareFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
