//! Original Go file: `mod/agi/error.go`
//! Package: `agi`; LOC: 51; SHA256: `84687914787430fdc51b3543db67f1e14f6167ec8b3c20f850612f2645b3d5a8`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/error.go", package: "agi", go_loc: 51, functions: 1, types: 0, sha256: "84687914787430fdc51b3543db67f1e14f6167ec8b3c20f850612f2645b3d5a8" };

pub const GO_IMPORTS: &[&str] = &[
    "html/template",
    "net/http",
    "os",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("RenderErrorTemplate", "g *Gateway", 18),
];

pub async fn gateway_rendererrortemplate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/error.go", function: "Gateway.RenderErrorTemplate" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
