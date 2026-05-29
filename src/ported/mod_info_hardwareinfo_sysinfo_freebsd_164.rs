//! Original Go file: `mod/info/hardwareinfo/sysinfo_freebsd.go`
//! Package: `hardwareinfo`; LOC: 234; SHA256: `8c27d1a89b5a59c0afb44c2f4b253b8f3693101a9c7a762e234f1db3b280ed8a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/hardwareinfo/sysinfo_freebsd.go", package: "hardwareinfo", go_loc: 234, functions: 9, types: 0, sha256: "8c27d1a89b5a59c0afb44c2f4b253b8f3693101a9c7a762e234f1db3b280ed8a" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os/exec",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetCPUFreq", "", 43),
    ("GetCPUModel", "", 63),
    ("GetCPUHardware", "", 76),
    ("GetCPUArch", "", 89),
    ("GetCPUInfo", "", 101),
    ("Ifconfig", "", 119),
    ("GetDriveStat", "", 144),
    ("GetUSB", "", 192),
    ("GetRamInfo", "", 219),
];

pub async fn getcpufreq(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetCPUFreq" })
}

pub async fn getcpumodel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetCPUModel" })
}

pub async fn getcpuhardware(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetCPUHardware" })
}

pub async fn getcpuarch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetCPUArch" })
}

pub async fn getcpuinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetCPUInfo" })
}

pub async fn ifconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "Ifconfig" })
}

pub async fn getdrivestat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetDriveStat" })
}

pub async fn getusb(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetUSB" })
}

pub async fn getraminfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_freebsd.go", function: "GetRamInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
