//! Original Go file: `mod/disk/raid/mdadm.go`
//! Package: `raid`; LOC: 318; SHA256: `48b72a6da6670e78548e4e08393b30b813f46023d9e337f6477e04300a5b9998`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/mdadm.go", package: "raid", go_loc: 318, functions: 8, types: 2, sha256: "48b72a6da6670e78548e4e08393b30b813f46023d9e337f6477e04300a5b9998" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "os/exec",
    "path/filepath",
    "sort",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("RAIDMember", "struct", 24),
    ("RAIDDevice", "struct", 30),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetDiskUUIDByPath", "m *Manager", 38),
    ("CreateRAIDDevice", "m *Manager", 60),
    ("GetRAIDDevicesFromProcMDStat", "m *Manager", 127),
    ("DiskIsFailed", "m *Manager", 233),
    ("FailDisk", "m *Manager", 251),
    ("RemoveDisk", "m *Manager", 269),
    ("AddDisk", "m *Manager", 286),
    ("GrowRAIDDevice", "m *Manager", 303),
];

pub async fn manager_getdiskuuidbypath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.GetDiskUUIDByPath" })
}

pub async fn manager_createraiddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.CreateRAIDDevice" })
}

pub async fn manager_getraiddevicesfromprocmdstat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.GetRAIDDevicesFromProcMDStat" })
}

pub async fn manager_diskisfailed(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.DiskIsFailed" })
}

pub async fn manager_faildisk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.FailDisk" })
}

pub async fn manager_removedisk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.RemoveDisk" })
}

pub async fn manager_adddisk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.AddDisk" })
}

pub async fn manager_growraiddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/mdadm.go", function: "Manager.GrowRAIDDevice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
