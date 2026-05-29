//! Original Go file: `mod/agi/agi.zip.go`
//! Package: `agi`; LOC: 1151; SHA256: `44295110e552b2252d0c069970c7b0a5952e26edf1aa8694248aaa2bc9fa0acf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.zip.go", package: "agi", go_loc: 1151, functions: 2, types: 0, sha256: "44295110e552b2252d0c069970c7b0a5952e26edf1aa8694248aaa2bc9fa0acf" };

pub const GO_IMPORTS: &[&str] = &[
    "archive/zip",
    "encoding/json",
    "errors",
    "github.com/mholt/archiver/v3",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "log",
    "os",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ZipLibRegister", "g *Gateway", 27),
    ("injectZipFileLibFunctions", "g *Gateway", 34),
];

pub async fn gateway_ziplibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.zip.go", function: "Gateway.ZipLibRegister" })
}

pub async fn gateway_injectzipfilelibfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.zip.go", function: "Gateway.injectZipFileLibFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
