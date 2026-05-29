//! Original Go file: `mod/disk/raid/raid_test.go`
//! Package: `raid_test`; LOC: 76; SHA256: `ca77865a394ead0fd753e7f2415e6da2df00e98b314c6adb698ec3f0c06910eb`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/raid_test.go", package: "raid_test", go_loc: 76, functions: 4, types: 0, sha256: "ca77865a394ead0fd753e7f2415e6da2df00e98b314c6adb698ec3f0c06910eb" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestRemoveRAIDFromConfig", "", 11),
    ("TestAddRAIDToConfig", "", 21),
    ("TestReadRAIDInfo", "", 31),
    ("TestCreateRAIDDevice", "", 45),
];

pub async fn testremoveraidfromconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid_test.go", function: "TestRemoveRAIDFromConfig" })
}

pub async fn testaddraidtoconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid_test.go", function: "TestAddRAIDToConfig" })
}

pub async fn testreadraidinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid_test.go", function: "TestReadRAIDInfo" })
}

pub async fn testcreateraiddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid_test.go", function: "TestCreateRAIDDevice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
