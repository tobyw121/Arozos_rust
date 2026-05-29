//! Original Go file: `mod/filesystem/metadata/raw_test.go`
//! Package: `metadata`; LOC: 241; SHA256: `080e263dccf635693157411b706f57f4e4335cf1e9277844c020c4d686fea375`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/raw_test.go", package: "metadata", go_loc: 241, functions: 4, types: 0, sha256: "080e263dccf635693157411b706f57f4e4335cf1e9277844c020c4d686fea375" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/binary",
    "os",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestDNGParsing", "", 10),
    ("debugParseTIFF", "", 53),
    ("debugParseIFD", "", 106),
    ("getTagName", "", 220),
];

pub async fn testdngparsing(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw_test.go", function: "TestDNGParsing" })
}

pub async fn debugparsetiff(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw_test.go", function: "debugParseTIFF" })
}

pub async fn debugparseifd(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw_test.go", function: "debugParseIFD" })
}

pub async fn gettagname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw_test.go", function: "getTagName" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
