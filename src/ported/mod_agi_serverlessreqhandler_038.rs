//! Original Go file: `mod/agi/serverlessReqHandler.go`
//! Package: `agi`; LOC: 70; SHA256: `f25be140a57b60a0ff4376244e82d59bdaa031861fffac44d5f830f041d4c8df`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/serverlessReqHandler.go", package: "agi", go_loc: 70, functions: 1, types: 0, sha256: "f25be140a57b60a0ff4376244e82d59bdaa031861fffac44d5f830f041d4c8df" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "io",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("injectServerlessFunctions", "g *Gateway", 19),
];

pub async fn gateway_injectserverlessfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/serverlessReqHandler.go", function: "Gateway.injectServerlessFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
