//! Original Go file: `hardware.power.go`
//! Package: `main`; LOC: 129; SHA256: `18ff958599e82dbb70331444d34a7dfba597ed3796693b73833ffaa12cf1693d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "hardware.power.go", package: "main", go_loc: 129, functions: 4, types: 0, sha256: "18ff958599e82dbb70331444d34a7dfba597ed3796693b73833ffaa12cf1693d" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "os/exec",
    "runtime",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HardwarePowerInit", "", 12),
    ("hardware_power_checkIfHardware", "", 32),
    ("hardware_power_poweroff", "", 40),
    ("hardware_power_restart", "", 82),
];

pub async fn hardwarepowerinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "hardware.power.go", function: "HardwarePowerInit" })
}

pub async fn hardware_power_checkifhardware(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "hardware.power.go", function: "hardware_power_checkIfHardware" })
}

pub async fn hardware_power_poweroff(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "hardware.power.go", function: "hardware_power_poweroff" })
}

pub async fn hardware_power_restart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "hardware.power.go", function: "hardware_power_restart" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
