//! Original Go file: `mod/iot/sonoff_s2x/utils.go`
//! Package: `sonoff_s2x`; LOC: 37; SHA256: `a7159fd4b44b2cbaef2762666d7d088e1fdeeb22006b37a0ab27eb0df5eb687d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/sonoff_s2x/utils.go", package: "sonoff_s2x", go_loc: 37, functions: 2, types: 0, sha256: "a7159fd4b44b2cbaef2762666d7d088e1fdeeb22006b37a0ab27eb0df5eb687d" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "io",
    "net/http",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("isJSON", "", 12),
    ("tryGet", "", 17),
];

pub async fn isjson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/utils.go", function: "isJSON" })
}

pub async fn tryget(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/utils.go", function: "tryGet" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
