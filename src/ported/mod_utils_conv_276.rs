//! Original Go file: `mod/utils/conv.go`
//! Package: `utils`; LOC: 16; SHA256: `f335abbdbfe2602299669b81c1e111e74cf5b3af01cc782d12284ccbc75934e0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/utils/conv.go", package: "utils", go_loc: 16, functions: 2, types: 0, sha256: "f335abbdbfe2602299669b81c1e111e74cf5b3af01cc782d12284ccbc75934e0" };

pub const GO_IMPORTS: &[&str] = &[
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("StringToInt64", "", 5),
    ("Int64ToString", "", 13),
];

pub async fn stringtoint64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/conv.go", function: "StringToInt64" })
}

pub async fn int64tostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/conv.go", function: "Int64ToString" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
