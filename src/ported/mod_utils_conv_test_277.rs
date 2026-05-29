//! Original Go file: `mod/utils/conv_test.go`
//! Package: `utils`; LOC: 69; SHA256: `869e9dc3a263b0243d2cfe2a7e559ca4f97ad6855a3a3d87fef57f8267644a3a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/utils/conv_test.go", package: "utils", go_loc: 69, functions: 2, types: 0, sha256: "869e9dc3a263b0243d2cfe2a7e559ca4f97ad6855a3a3d87fef57f8267644a3a" };

pub const GO_IMPORTS: &[&str] = &[
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestStringToInt64", "", 7),
    ("TestInt64ToString", "", 39),
];

pub async fn teststringtoint64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/conv_test.go", function: "TestStringToInt64" })
}

pub async fn testint64tostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/conv_test.go", function: "TestInt64ToString" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
