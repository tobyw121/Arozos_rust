//! Original Go file: `mod/disk/diskcapacity/dftool/dftool.go`
//! Package: `dftool`; LOC: 96; SHA256: `cadda68fbf32f7bda914b9a3dc8e74ca42986eaa538dd3c73e58d122deb17517`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/diskcapacity/dftool/dftool.go", package: "dftool", go_loc: 96, functions: 1, types: 1, sha256: "cadda68fbf32f7bda914b9a3dc8e74ca42986eaa538dd3c73e58d122deb17517" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/disk/diskspace",
    "os/exec",
    "path/filepath",
    "runtime",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Capacity", "struct", 14),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetCapacityInfoFromPath", "", 21),
];

pub async fn getcapacityinfofrompath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskcapacity/dftool/dftool.go", function: "GetCapacityInfoFromPath" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
