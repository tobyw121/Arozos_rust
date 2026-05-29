//! Original Go file: `mod/disk/diskfs/diskfs.go`
//! Package: `diskfs`; LOC: 276; SHA256: `97beb5f5581bfeec48a61f158bf65a4e12dd47f84d8c40a85d89de43fb724ebf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/diskfs/diskfs.go", package: "diskfs", go_loc: 276, functions: 10, types: 3, sha256: "97beb5f5581bfeec48a61f158bf65a4e12dd47f84d8c40a85d89de43fb724ebf" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "bytes",
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "os/exec",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("PartitionMeta", "struct", 24),
    ("BlockDeviceMeta", "struct", 40),
    ("StorageDevicesMeta", "struct", 58),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FormatPackageInstalled", "", 64),
    ("FormatStorageDevice", "", 69),
    ("ListAllStorageDevices", "", 115),
    ("GetBlockDeviceMeta", "", 124),
    ("GetDiskUUID", "", 151),
    ("GetPartitionMeta", "", 164),
    ("DeviceIsMounted", "", 197),
    ("UnmountDevice", "", 229),
    ("ForceUnmountDevice", "", 245),
    ("WipeDisk", "", 259),
];

pub async fn formatpackageinstalled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "FormatPackageInstalled" })
}

pub async fn formatstoragedevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "FormatStorageDevice" })
}

pub async fn listallstoragedevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "ListAllStorageDevices" })
}

pub async fn getblockdevicemeta(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "GetBlockDeviceMeta" })
}

pub async fn getdiskuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "GetDiskUUID" })
}

pub async fn getpartitionmeta(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "GetPartitionMeta" })
}

pub async fn deviceismounted(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "DeviceIsMounted" })
}

pub async fn unmountdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "UnmountDevice" })
}

pub async fn forceunmountdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "ForceUnmountDevice" })
}

pub async fn wipedisk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/diskfs.go", function: "WipeDisk" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
