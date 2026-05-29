//! Original Go file: `mod/agi/agi.appdata.go`
//! Package: `agi`; LOC: 133; SHA256: `1af403a4693ca68e2531adfdf4227fbb18bb25525f16397283c8bc437a11bdaa`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.appdata.go", package: "agi", go_loc: 133, functions: 2, types: 0, sha256: "1af403a4693ca68e2531adfdf4227fbb18bb25525f16397283c8bc437a11bdaa" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("AppdataLibRegister", "g *Gateway", 28),
    ("injectAppdataLibFunctions", "g *Gateway", 35),
];

pub async fn gateway_appdatalibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appdata.go", function: "Gateway.AppdataLibRegister" })
}

pub async fn gateway_injectappdatalibfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appdata.go", function: "Gateway.injectAppdataLibFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
