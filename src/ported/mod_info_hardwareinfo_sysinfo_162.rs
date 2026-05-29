//! Original Go file: `mod/info/hardwareinfo/sysinfo.go`
//! Package: `hardwareinfo`; LOC: 202; SHA256: `69662b28a6961f772c4dcd379d22886a4517c4b58e41f28ed9f2646f8a093725`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/hardwareinfo/sysinfo.go", package: "hardwareinfo", go_loc: 202, functions: 5, types: 0, sha256: "69662b28a6961f772c4dcd379d22886a4517c4b58e41f28ed9f2646f8a093725" };

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
    ("Ifconfig", "", 16),
    ("GetDriveStat", "", 40),
    ("GetUSB", "", 85),
    ("GetCPUInfo", "", 108),
    ("GetRamInfo", "", 185),
];

pub async fn ifconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo.go", function: "Ifconfig" })
}

pub async fn getdrivestat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo.go", function: "GetDriveStat" })
}

pub async fn getusb(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo.go", function: "GetUSB" })
}

pub async fn getcpuinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo.go", function: "GetCPUInfo" })
}

pub async fn getraminfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo.go", function: "GetRamInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
