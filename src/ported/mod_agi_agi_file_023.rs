//! Original Go file: `mod/agi/agi.file.go`
//! Package: `agi`; LOC: 869; SHA256: `bffcbc67ba28bacd3cf8d0cc6ae3d425179afa891162d80996582a4de0cf1303`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.file.go", package: "agi", go_loc: 869, functions: 2, types: 0, sha256: "bffcbc67ba28bacd3cf8d0cc6ae3d425179afa891162d80996582a4de0cf1303" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/md5",
    "encoding/hex",
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/filesystem/fssort",
    "imuslab.com/arozos/mod/filesystem/hidden",
    "io",
    "io/fs",
    "log",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FileLibRegister", "g *Gateway", 30),
    ("injectFileLibFunctions", "g *Gateway", 37),
];

pub async fn gateway_filelibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.file.go", function: "Gateway.FileLibRegister" })
}

pub async fn gateway_injectfilelibfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.file.go", function: "Gateway.injectFileLibFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
