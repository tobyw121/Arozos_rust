//! Original Go file: `mod/disk/diskfs/devices_linux.go`
//! Package: `diskfs`; LOC: 429; SHA256: `b516bc81a27a12897bfcf9982034d88bc774fd5ceac170fee28793b3046710fd`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/diskfs/devices_linux.go", package: "diskfs", go_loc: 429, functions: 20, types: 2, sha256: "b516bc81a27a12897bfcf9982034d88bc774fd5ceac170fee28793b3046710fd" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "bytes",
    "fmt",
    "os",
    "os/exec",
    "path/filepath",
    "sort",
    "strconv",
    "strings",
    "unicode",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("blkidMeta", "struct", 16),
    ("dfStat", "struct", 22),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("listStorageDevicesFromFdisk", "", 27),
    ("fdiskEnumerateDeviceNames", "", 84),
    ("buildStorageDeviceMeta", "", 133),
    ("partitionMetaFromBlockDevice", "", 169),
    ("fdiskBinary", "", 186),
    ("shouldIgnoreStorageDevice", "", 196),
    ("detectDeviceType", "", 211),
    ("rootDeviceName", "", 227),
    ("hasOnlyDigits", "", 246),
    ("readMounts", "", 258),
    ("readDFStats", "", 280),
    ("readBlkidMeta", "", 316),
    ("parseBlkidField", "", 342),
    ("findDeviceMountpoint", "", 356),
    ("canonicalDevice", "", 363),
    ("readTrimmedFile", "", 377),
    ("readBoolFile", "", 385),
    ("readDeviceSize", "", 394),
    ("readDeviceModel", "", 402),
    ("formatByteCountIEC", "", 416),
];

pub async fn liststoragedevicesfromfdisk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "listStorageDevicesFromFdisk" })
}

pub async fn fdiskenumeratedevicenames(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "fdiskEnumerateDeviceNames" })
}

pub async fn buildstoragedevicemeta(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "buildStorageDeviceMeta" })
}

pub async fn partitionmetafromblockdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "partitionMetaFromBlockDevice" })
}

pub async fn fdiskbinary(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "fdiskBinary" })
}

pub async fn shouldignorestoragedevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "shouldIgnoreStorageDevice" })
}

pub async fn detectdevicetype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "detectDeviceType" })
}

pub async fn rootdevicename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "rootDeviceName" })
}

pub async fn hasonlydigits(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "hasOnlyDigits" })
}

pub async fn readmounts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readMounts" })
}

pub async fn readdfstats(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readDFStats" })
}

pub async fn readblkidmeta(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readBlkidMeta" })
}

pub async fn parseblkidfield(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "parseBlkidField" })
}

pub async fn finddevicemountpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "findDeviceMountpoint" })
}

pub async fn canonicaldevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "canonicalDevice" })
}

pub async fn readtrimmedfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readTrimmedFile" })
}

pub async fn readboolfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readBoolFile" })
}

pub async fn readdevicesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readDeviceSize" })
}

pub async fn readdevicemodel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "readDeviceModel" })
}

pub async fn formatbytecountiec(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskfs/devices_linux.go", function: "formatByteCountIEC" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
