//! Original Go file: `mod/disk/raid/raidutils.go`
//! Package: `raid`; LOC: 291; SHA256: `1319ec1693dafa5d2585b5d2ec4ad05fcc9f933c27bb802912d13e9d37d62593`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/raidutils.go", package: "raid", go_loc: 291, functions: 14, types: 0, sha256: "1319ec1693dafa5d2585b5d2ec4ad05fcc9f933c27bb802912d13e9d37d62593" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/disk/diskfs",
    "os",
    "os/exec",
    "path/filepath",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetNextAvailableMDDevice", "", 17),
    ("IsSafeToRemove", "m *Manager", 29),
    ("DiskIsUsedInAnotherRAIDVol", "m *Manager", 64),
    ("DiskIsRoot", "m *Manager", 82),
    ("ClearSuperblock", "m *Manager", 98),
    ("RestartRAIDService", "m *Manager", 122),
    ("StopRAIDDevice", "m *Manager", 139),
    ("RemoveRAIDMember", "m *Manager", 152),
    ("IsValidRAIDLevel", "", 167),
    ("GetRAIDDeviceByDevicePath", "m *Manager", 186),
    ("RAIDDeviceExists", "m *Manager", 207),
    ("RAIDArrayContainsFailedDisks", "m *Manager", 213),
    ("GetRAIDPartitionSize", "", 222),
    ("GetRAIDUsedSize", "", 258),
];

pub async fn getnextavailablemddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "GetNextAvailableMDDevice" })
}

pub async fn manager_issafetoremove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.IsSafeToRemove" })
}

pub async fn manager_diskisusedinanotherraidvol(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.DiskIsUsedInAnotherRAIDVol" })
}

pub async fn manager_diskisroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.DiskIsRoot" })
}

pub async fn manager_clearsuperblock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.ClearSuperblock" })
}

pub async fn manager_restartraidservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.RestartRAIDService" })
}

pub async fn manager_stopraiddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.StopRAIDDevice" })
}

pub async fn manager_removeraidmember(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.RemoveRAIDMember" })
}

pub async fn isvalidraidlevel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "IsValidRAIDLevel" })
}

pub async fn manager_getraiddevicebydevicepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.GetRAIDDeviceByDevicePath" })
}

pub async fn manager_raiddeviceexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.RAIDDeviceExists" })
}

pub async fn manager_raidarraycontainsfaileddisks(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "Manager.RAIDArrayContainsFailedDisks" })
}

pub async fn getraidpartitionsize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "GetRAIDPartitionSize" })
}

pub async fn getraidusedsize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raidutils.go", function: "GetRAIDUsedSize" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
