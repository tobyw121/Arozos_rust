//! Original Go file: `mod/disk/diskmg/diskmg.go`
//! Package: `diskmg`; LOC: 504; SHA256: `fdf5db3d4080c6f91180000d42fab9a95925e3c44674cff9be74376ec5913538`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/diskmg/diskmg.go", package: "diskmg", go_loc: 504, functions: 14, types: 6, sha256: "fdf5db3d4080c6f91180000d42fab9a95925e3c44674cff9be74376ec5913538" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/disk/diskfs",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "os/exec",
    "path/filepath",
    "regexp",
    "runtime",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Lsblk", "struct", 21),
    ("LsblkF", "struct", 25),
    ("LsblkPartition", "struct", 29),
    ("LsblkDevice", "struct", 39),
    ("LsblkFPartition", "struct", 50),
    ("LsblkFDevice", "struct", 60),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleView", "", 83),
    ("HandleMount", "", 160),
    ("HandleFormat", "", 235),
    ("Mount", "", 327),
    ("Unmount", "", 346),
    ("HandleListMountPoints", "", 361),
    ("checkDeviceMounted", "", 368),
    ("getDeviceMountPoint", "", 373),
    ("checkDeviceValid", "", 397),
    ("HandlePlatform", "", 410),
    ("buildLegacyLinuxDiskViews", "", 415),
    ("readLegacyDFTable", "", 473),
    ("nilIfEmpty", "", 491),
    ("nilIfZero", "", 499),
];

pub async fn handleview(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "HandleView" })
}

pub async fn handlemount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "HandleMount" })
}

pub async fn handleformat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "HandleFormat" })
}

pub async fn mount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "Mount" })
}

pub async fn unmount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "Unmount" })
}

pub async fn handlelistmountpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "HandleListMountPoints" })
}

pub async fn checkdevicemounted(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "checkDeviceMounted" })
}

pub async fn getdevicemountpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "getDeviceMountPoint" })
}

pub async fn checkdevicevalid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "checkDeviceValid" })
}

pub async fn handleplatform(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "HandlePlatform" })
}

pub async fn buildlegacylinuxdiskviews(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "buildLegacyLinuxDiskViews" })
}

pub async fn readlegacydftable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "readLegacyDFTable" })
}

pub async fn nilifempty(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "nilIfEmpty" })
}

pub async fn nilifzero(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskmg/diskmg.go", function: "nilIfZero" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
