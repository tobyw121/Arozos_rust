//! Original Go file: `mod/agi/agi.ffmpeg.go`
//! Package: `agi`; LOC: 148; SHA256: `ff8b08aa8a4b6b0a5fbd25c987bd6822dc0bea51c487e11755d6599ae99a57d0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.ffmpeg.go", package: "agi", go_loc: 148, functions: 2, types: 0, sha256: "ff8b08aa8a4b6b0a5fbd25c987bd6822dc0bea51c487e11755d6599ae99a57d0" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/robertkrimen/otto",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/agi/static/ffmpegutil",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FFmpegLibRegister", "g *Gateway", 26),
    ("injectFFmpegFunctions", "g *Gateway", 33),
];

pub async fn gateway_ffmpeglibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.ffmpeg.go", function: "Gateway.FFmpegLibRegister" })
}

pub async fn gateway_injectffmpegfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.ffmpeg.go", function: "Gateway.injectFFmpegFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
