//! Original Go file: `mod/disk/raid/losetup.go`
//! Package: `raid`; LOC: 135; SHA256: `3b7a241e2e4a235d6e72e22f28ce572bfcd72e4c0187c9838168f3ece25cd54d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/losetup.go", package: "raid", go_loc: 135, functions: 6, types: 1, sha256: "3b7a241e2e4a235d6e72e22f28ce572bfcd72e4c0187c9838168f3ece25cd54d" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "os",
    "os/exec",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("LoopDevice", "struct", 17),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ListAllLoopDevices", "", 24),
    ("MountImageAsLoopDevice", "", 56),
    ("UnmountLoopDeviceByImagePath", "", 71),
    ("UnmountLoopDeviceByID", "", 92),
    ("GetLoopDriveIDFromImagePath", "", 107),
    ("ImageMountedAsLoopDevice", "", 129),
];

pub async fn listallloopdevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/losetup.go", function: "ListAllLoopDevices" })
}

pub async fn mountimageasloopdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/losetup.go", function: "MountImageAsLoopDevice" })
}

pub async fn unmountloopdevicebyimagepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/losetup.go", function: "UnmountLoopDeviceByImagePath" })
}

pub async fn unmountloopdevicebyid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/losetup.go", function: "UnmountLoopDeviceByID" })
}

pub async fn getloopdriveidfromimagepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/losetup.go", function: "GetLoopDriveIDFromImagePath" })
}

pub async fn imagemountedasloopdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/losetup.go", function: "ImageMountedAsLoopDevice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
