//! Original Go file: `mod/info/hardwareinfo/sysinfo_window.go`
//! Package: `hardwareinfo`; LOC: 98; SHA256: `5c70e3ad10e093da9ee62a96e32070bab061042189c31d888005dc5b985d0584`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/hardwareinfo/sysinfo_window.go", package: "hardwareinfo", go_loc: 98, functions: 5, types: 0, sha256: "5c70e3ad10e093da9ee62a96e32070bab061042189c31d888005dc5b985d0584" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetCPUInfo", "", 15),
    ("Ifconfig", "", 33),
    ("GetDriveStat", "", 47),
    ("GetUSB", "", 71),
    ("GetRamInfo", "", 85),
];

pub async fn getcpuinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_window.go", function: "GetCPUInfo" })
}

pub async fn ifconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_window.go", function: "Ifconfig" })
}

pub async fn getdrivestat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_window.go", function: "GetDriveStat" })
}

pub async fn getusb(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_window.go", function: "GetUSB" })
}

pub async fn getraminfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_window.go", function: "GetRamInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
