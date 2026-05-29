//! Original Go file: `disk_alpnas.go`
//! Package: `main`; LOC: 1027; SHA256: `c24712e1235fcd5701630434a654859e4279653f9aedf1ee55db38f7ea53e7a2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "disk_alpnas.go", package: "main", go_loc: 1027, functions: 38, types: 2, sha256: "c24712e1235fcd5701630434a654859e4279653f9aedf1ee55db38f7ea53e7a2" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/modules",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "os",
    "os/exec",
    "path/filepath",
    "runtime",
    "sort",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("alpnasDriveRecord", "struct", 22),
    ("alpnasDriveListResponse", "struct", 48),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("alpnasDriveServiceInit", "", 56),
    ("alpnasHandleDriveList", "", 84),
    ("alpnasHandleDriveRescan", "", 110),
    ("alpnasHandleDriveMount", "", 134),
    ("alpnasHandleDriveUnmount", "", 164),
    ("alpnasRequireAdmin", "", 203),
    ("alpnasCollectDriveRecords", "", 218),
    ("alpnasCollectStateRecords", "", 256),
    ("alpnasStateBool", "", 362),
    ("alpnasCollectFdiskRecords", "", 371),
    ("alpnasParseFdiskDiskLine", "", 422),
    ("alpnasParseFdiskPartitionLine", "", 431),
    ("alpnasExtractLeadingUint64", "", 446),
    ("alpnasBuildRecordFromDevice", "", 456),
    ("alpnasResolveOpenPath", "", 529),
    ("alpnasCleanMountPath", "", 571),
    ("alpnasCanMountDevice", "", 579),
    ("alpnasCanUnmountPath", "", 604),
    ("alpnasIsSystemMount", "", 611),
    ("alpnasIsReservedFilesystem", "", 622),
    ("alpnasReadBlkidValue", "", 631),
    ("alpnasCmdlineMatches", "", 683),
    ("alpnasLookupMountpoint", "", 701),
    ("alpnasIsBootMedium", "", 724),
    ("alpnasRunDriveCtl", "", 767),
    ("alpnasRunStorageSync", "", 787),
    ("alpnasResolveCurrentMountpoint", "", 807),
    ("alpnasUserIsAdmin", "", 833),
    ("alpnasCommandEnv", "", 849),
    ("alpnasLookupBinary", "", 853),
    ("alpnasSupportsLegacyDiskManager", "", 871),
    ("alpnasCanonicalDevice", "", 875),
    ("alpnasIsReservedDevice", "", 886),
    ("alpnasDeviceHasChildPartitions", "", 900),
    ("alpnasDeviceRootDisk", "", 922),
    ("alpnasBootSources", "", 934),
    ("alpnasCollectMountedFallback", "", 965),
    ("alpnasEscapeJSONString", "", 1023),
];

pub async fn alpnasdriveserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasDriveServiceInit" })
}

pub async fn alpnashandledrivelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasHandleDriveList" })
}

pub async fn alpnashandledriverescan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasHandleDriveRescan" })
}

pub async fn alpnashandledrivemount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasHandleDriveMount" })
}

pub async fn alpnashandledriveunmount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasHandleDriveUnmount" })
}

pub async fn alpnasrequireadmin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasRequireAdmin" })
}

pub async fn alpnascollectdriverecords(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCollectDriveRecords" })
}

pub async fn alpnascollectstaterecords(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCollectStateRecords" })
}

pub async fn alpnasstatebool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasStateBool" })
}

pub async fn alpnascollectfdiskrecords(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCollectFdiskRecords" })
}

pub async fn alpnasparsefdiskdiskline(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasParseFdiskDiskLine" })
}

pub async fn alpnasparsefdiskpartitionline(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasParseFdiskPartitionLine" })
}

pub async fn alpnasextractleadinguint64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasExtractLeadingUint64" })
}

pub async fn alpnasbuildrecordfromdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasBuildRecordFromDevice" })
}

pub async fn alpnasresolveopenpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasResolveOpenPath" })
}

pub async fn alpnascleanmountpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCleanMountPath" })
}

pub async fn alpnascanmountdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCanMountDevice" })
}

pub async fn alpnascanunmountpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCanUnmountPath" })
}

pub async fn alpnasissystemmount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasIsSystemMount" })
}

pub async fn alpnasisreservedfilesystem(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasIsReservedFilesystem" })
}

pub async fn alpnasreadblkidvalue(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasReadBlkidValue" })
}

pub async fn alpnascmdlinematches(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCmdlineMatches" })
}

pub async fn alpnaslookupmountpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasLookupMountpoint" })
}

pub async fn alpnasisbootmedium(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasIsBootMedium" })
}

pub async fn alpnasrundrivectl(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasRunDriveCtl" })
}

pub async fn alpnasrunstoragesync(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasRunStorageSync" })
}

pub async fn alpnasresolvecurrentmountpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasResolveCurrentMountpoint" })
}

pub async fn alpnasuserisadmin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasUserIsAdmin" })
}

pub async fn alpnascommandenv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCommandEnv" })
}

pub async fn alpnaslookupbinary(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasLookupBinary" })
}

pub async fn alpnassupportslegacydiskmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasSupportsLegacyDiskManager" })
}

pub async fn alpnascanonicaldevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCanonicalDevice" })
}

pub async fn alpnasisreserveddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasIsReservedDevice" })
}

pub async fn alpnasdevicehaschildpartitions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasDeviceHasChildPartitions" })
}

pub async fn alpnasdevicerootdisk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasDeviceRootDisk" })
}

pub async fn alpnasbootsources(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasBootSources" })
}

pub async fn alpnascollectmountedfallback(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasCollectMountedFallback" })
}

pub async fn alpnasescapejsonstring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk_alpnas.go", function: "alpnasEscapeJSONString" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
