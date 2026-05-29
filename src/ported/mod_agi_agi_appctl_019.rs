//! Original Go file: `mod/agi/agi.appctl.go`
//! Package: `agi`; LOC: 180; SHA256: `a2191ecbdf8d92ad31e3a49fde068f08cb559cb9b7ba275a2851d162e3016ee6`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.appctl.go", package: "agi", go_loc: 180, functions: 5, types: 0, sha256: "a2191ecbdf8d92ad31e3a49fde068f08cb559cb9b7ba275a2851d162e3016ee6" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "log",
    "os/exec",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("AppCtlLibRegister", "g *Gateway", 16),
    ("injectAppCtlFunctions", "g *Gateway", 23),
    ("runAppCtl", "", 96),
    ("extractAppCtlJSON", "", 133),
    ("jsonValue", "", 173),
];

pub async fn gateway_appctllibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appctl.go", function: "Gateway.AppCtlLibRegister" })
}

pub async fn gateway_injectappctlfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appctl.go", function: "Gateway.injectAppCtlFunctions" })
}

pub async fn runappctl(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appctl.go", function: "runAppCtl" })
}

pub async fn extractappctljson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appctl.go", function: "extractAppCtlJSON" })
}

pub async fn jsonvalue(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.appctl.go", function: "jsonValue" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
