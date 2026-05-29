//! Original Go file: `mod/agi/moduleManager.go`
//! Package: `agi`; LOC: 68; SHA256: `f921f87b3ae55897b1a0b10d0abdfd4b3f26df6eb54a69d8d67fd800570ddfab`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/moduleManager.go", package: "agi", go_loc: 68, functions: 2, types: 2, sha256: "f921f87b3ae55897b1a0b10d0abdfd4b3f26df6eb54a69d8d67fd800570ddfab" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/apt",
    "log",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("AgiLibInjectionIntergface", "func", 23),
    ("AgiLibInterface", "interface", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("RegisterLib", "g *Gateway", 31),
    ("LoadAllFunctionalModules", "g *Gateway", 48),
];

pub async fn gateway_registerlib(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/moduleManager.go", function: "Gateway.RegisterLib" })
}

pub async fn gateway_loadallfunctionalmodules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/moduleManager.go", function: "Gateway.LoadAllFunctionalModules" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
