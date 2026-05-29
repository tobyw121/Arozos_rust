//! Original Go file: `mod/info/usageinfo/cpu.go`
//! Package: `usageinfo`; LOC: 97; SHA256: `ea12309414f669f10f81f89d9662b7f0b9eb85c5825b94659edf5ca1ef3cf462`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/usageinfo/cpu.go", package: "usageinfo", go_loc: 97, functions: 3, types: 1, sha256: "ea12309414f669f10f81f89d9662b7f0b9eb85c5825b94659edf5ca1ef3cf462" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "io/ioutil",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("CPUStats", "struct", 11),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getCPUStats", "", 22),
    ("calculateCPUUsage", "", 61),
    ("GetCPUUsageUsingProcStat", "", 78),
];

pub async fn getcpustats(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/usageinfo/cpu.go", function: "getCPUStats" })
}

pub async fn calculatecpuusage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/usageinfo/cpu.go", function: "calculateCPUUsage" })
}

pub async fn getcpuusageusingprocstat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/usageinfo/cpu.go", function: "GetCPUUsageUsingProcStat" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
