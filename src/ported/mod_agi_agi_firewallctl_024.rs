//! Original Go file: `mod/agi/agi.firewallctl.go`
//! Package: `agi`; LOC: 119; SHA256: `f2c82db2c42ee4b1b63450986415279774f089d96bd057034b5500534bd3f2f2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.firewallctl.go", package: "agi", go_loc: 119, functions: 3, types: 0, sha256: "f2c82db2c42ee4b1b63450986415279774f089d96bd057034b5500534bd3f2f2" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
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
    ("FirewallCtlLibRegister", "g *Gateway", 15),
    ("injectFirewallCtlFunctions", "g *Gateway", 22),
    ("runFirewallCtl", "", 99),
];

pub async fn gateway_firewallctllibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.firewallctl.go", function: "Gateway.FirewallCtlLibRegister" })
}

pub async fn gateway_injectfirewallctlfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.firewallctl.go", function: "Gateway.injectFirewallCtlFunctions" })
}

pub async fn runfirewallctl(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.firewallctl.go", function: "runFirewallCtl" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
