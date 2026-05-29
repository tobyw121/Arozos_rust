//! Original Go file: `mod/info/hardwareinfo/sysinfo_darwin.go`
//! Package: `hardwareinfo`; LOC: 237; SHA256: `272b76f77a3c46e4a4c89071919519ffc949f0bd506e2b74632b874735a6eba0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/hardwareinfo/sysinfo_darwin.go", package: "hardwareinfo", go_loc: 237, functions: 9, types: 0, sha256: "272b76f77a3c46e4a4c89071919519ffc949f0bd506e2b74632b874735a6eba0" };

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
    ("GetCPUFreq", "", 46),
    ("GetCPUModel", "", 66),
    ("GetCPUHardware", "", 79),
    ("GetCPUArch", "", 92),
    ("GetCPUInfo", "", 104),
    ("Ifconfig", "", 122),
    ("GetDriveStat", "", 147),
    ("GetUSB", "", 195),
    ("GetRamInfo", "", 222),
];

pub async fn getcpufreq(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetCPUFreq" })
}

pub async fn getcpumodel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetCPUModel" })
}

pub async fn getcpuhardware(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetCPUHardware" })
}

pub async fn getcpuarch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetCPUArch" })
}

pub async fn getcpuinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetCPUInfo" })
}

pub async fn ifconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "Ifconfig" })
}

pub async fn getdrivestat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetDriveStat" })
}

pub async fn getusb(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetUSB" })
}

pub async fn getraminfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/sysinfo_darwin.go", function: "GetRamInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
