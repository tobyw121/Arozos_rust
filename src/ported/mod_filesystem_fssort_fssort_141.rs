//! Original Go file: `mod/filesystem/fssort/fssort.go`
//! Package: `fssort`; LOC: 145; SHA256: `6894a2d110c216800b291cfe53f2ea57213e8b552cd2b6ba9bcbf37067bc1597`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/fssort/fssort.go", package: "fssort", go_loc: 145, functions: 4, types: 1, sha256: "6894a2d110c216800b291cfe53f2ea57213e8b552cd2b6ba9bcbf37067bc1597" };

pub const GO_IMPORTS: &[&str] = &[
    "io/fs",
    "path/filepath",
    "sort",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("sortBufferedStructure", "struct", 10),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SortFileList", "", 22),
    ("SortDirEntryList", "", 108),
    ("SortModeIsSupported", "", 133),
    ("contains", "", 137),
];

pub async fn sortfilelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fssort/fssort.go", function: "SortFileList" })
}

pub async fn sortdirentrylist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fssort/fssort.go", function: "SortDirEntryList" })
}

pub async fn sortmodeissupported(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fssort/fssort.go", function: "SortModeIsSupported" })
}

pub async fn contains(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fssort/fssort.go", function: "contains" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
