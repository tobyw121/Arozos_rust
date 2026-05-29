//! Original Go file: `mod/disk/raid/mdadmConf.go`
//! Package: `raid`; LOC: 205; SHA256: `ab4ebf5d6f7b943db2860d706c9f2302310aef93a37fb98f94bb7468113fd37a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/mdadmConf.go", package: "raid", go_loc: 205, functions: 4, types: 0, sha256: "ab4ebf5d6f7b943db2860d706c9f2302310aef93a37fb98f94bb7468113fd37a" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "imuslab.com/arozos/mod/disk/diskfs",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "os/exec",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FlushReload", "m *Manager", 26),
    ("removeDevicesEntry", "", 66),
    ("UpdateMDADMConfig", "m *Manager", 88),
    ("RemoveVolumeFromMDADMConfig", "m *Manager", 193),
];

pub async fn manager_flushreload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadmConf.go", function: "Manager.FlushReload" })
}

pub async fn removedevicesentry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadmConf.go", function: "removeDevicesEntry" })
}

pub async fn manager_updatemdadmconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadmConf.go", function: "Manager.UpdateMDADMConfig" })
}

pub async fn manager_removevolumefrommdadmconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadmConf.go", function: "Manager.RemoveVolumeFromMDADMConfig" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
