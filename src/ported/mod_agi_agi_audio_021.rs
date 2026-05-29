//! Original Go file: `mod/agi/agi.audio.go`
//! Package: `agi`; LOC: 34; SHA256: `f5588d89c1b28ffc50a7f74554b0c66807f68d2c39fda9bd6b80834317926fee`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.audio.go", package: "agi", go_loc: 34, functions: 2, types: 0, sha256: "f5588d89c1b28ffc50a7f74554b0c66807f68d2c39fda9bd6b80834317926fee" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/agi/static",
    "log",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("AudioLibRegister", "g *Gateway", 19),
    ("injectAudioFunctions", "g *Gateway", 26),
];

pub async fn gateway_audiolibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.audio.go", function: "Gateway.AudioLibRegister" })
}

pub async fn gateway_injectaudiofunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.audio.go", function: "Gateway.injectAudioFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
