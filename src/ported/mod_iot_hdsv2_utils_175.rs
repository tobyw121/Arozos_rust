//! Original Go file: `mod/iot/hdsv2/utils.go`
//! Package: `hdsv2`; LOC: 37; SHA256: `f2b707a98dfa7e7fa57626dda7aee5ebf75741ead9aa3d38ad8ffcfc030c337a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/hdsv2/utils.go", package: "hdsv2", go_loc: 37, functions: 2, types: 0, sha256: "f2b707a98dfa7e7fa57626dda7aee5ebf75741ead9aa3d38ad8ffcfc030c337a" };

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
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/utils.go", function: "isJSON" })
}

pub async fn tryget(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/utils.go", function: "tryGet" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
