//! Original Go file: `mod/info/usageinfo/usageinfo.go`
//! Package: `usageinfo`; LOC: 387; SHA256: `3b721ffe89b0c1d742c42d60a108d0797980619d06746173433d458679d9f86d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/usageinfo/usageinfo.go", package: "usageinfo", go_loc: 387, functions: 3, types: 0, sha256: "3b721ffe89b0c1d742c42d60a108d0797980619d06746173433d458679d9f86d" };

pub const GO_IMPORTS: &[&str] = &[
    "math",
    "os/exec",
    "runtime",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetCPUUsage", "", 26),
    ("GetNumericRAMUsage", "", 125),
    ("GetRAMUsage", "", 258),
];

pub async fn getcpuusage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/usageinfo/usageinfo.go", function: "GetCPUUsage" })
}

pub async fn getnumericramusage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/usageinfo/usageinfo.go", function: "GetNumericRAMUsage" })
}

pub async fn getramusage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/usageinfo/usageinfo.go", function: "GetRAMUsage" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
