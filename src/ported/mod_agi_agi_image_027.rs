//! Original Go file: `mod/agi/agi.image.go`
//! Package: `agi`; LOC: 612; SHA256: `4b7cd28c8e65201058e01701099d8474d7ee7fe17b5c1691b23fe52f354ae384`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.image.go", package: "agi", go_loc: 612, functions: 2, types: 0, sha256: "4b7cd28c8e65201058e01701099d8474d7ee7fe17b5c1691b23fe52f354ae384" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "encoding/base64",
    "encoding/json",
    "errors",
    "fmt",
    "github.com/disintegration/imaging",
    "github.com/oliamb/cutter",
    "github.com/robertkrimen/otto",
    "github.com/rwcarlsen/goexif/exif",
    "image",
    "image/jpeg",
    "image/png",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/utils",
    "io",
    "log",
    "os",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ImageLibRegister", "g *Gateway", 37),
    ("injectImageLibFunctions", "g *Gateway", 44),
];

pub async fn gateway_imagelibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.image.go", function: "Gateway.ImageLibRegister" })
}

pub async fn gateway_injectimagelibfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.image.go", function: "Gateway.injectImageLibFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
