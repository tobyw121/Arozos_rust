//! Original Go file: `mod/filesystem/config.go`
//! Package: `filesystem`; LOC: 74; SHA256: `5df4f03b19b4db69581ff8209303a66dcbd19739f079fc977c4a72af8cde03f3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/config.go", package: "filesystem", go_loc: 74, functions: 2, types: 1, sha256: "5df4f03b19b4db69581ff8209303a66dcbd19739f079fc977c4a72af8cde03f3" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/filesystem/arozfs",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FileSystemOption", "struct", 11),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("loadConfigFromJSON", "", 27),
    ("ValidateOption", "", 35),
];

pub async fn loadconfigfromjson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/config.go", function: "loadConfigFromJSON" })
}

pub async fn validateoption(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/config.go", function: "ValidateOption" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
