//! Original Go file: `installer_alpnas.go`
//! Package: `main`; LOC: 551; SHA256: `72f97630bd3b3ab45bc52b5705adbb0a7149815cbb9a97b9e498f8307a5b1e78`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "installer_alpnas.go", package: "main", go_loc: 551, functions: 22, types: 3, sha256: "72f97630bd3b3ab45bc52b5705adbb0a7149815cbb9a97b9e498f8307a5b1e78" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "encoding/json",
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
    ("alpnasInstallerTarget", "struct", 19),
    ("alpnasInstallerTargetsResponse", "struct", 34),
    ("alpnasInstallerStateResponse", "struct", 39),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("alpnasInstallerServiceInit", "", 57),
    ("alpnasHandleInstallerState", "", 76),
    ("alpnasHandleInstallerTargets", "", 105),
    ("alpnasHandleInstallerStart", "", 118),
    ("alpnasInstallerAvailable", "", 178),
    ("alpnasIsLiveMode", "", 189),
    ("alpnasCollectInstallerTargets", "", 211),
    ("alpnasInstallerBlockDiskCandidates", "", 272),
    ("alpnasInstallerDeviceSize", "", 326),
    ("alpnasInstallerDiskIsBootMedium", "", 347),
    ("alpnasInstallerDeviceName", "", 363),
    ("alpnasInstallerReadSysfs", "", 375),
    ("alpnasInstallerSysfsBool", "", 387),
    ("alpnasInstallerTransport", "", 396),
    ("alpnasInstallerIsHotplug", "", 420),
    ("alpnasInstallerPartitionDevices", "", 432),
    ("alpnasInstallerPartitionCount", "", 458),
    ("alpnasInstallerMountedChildren", "", 462),
    ("alpnasInstallerFilesystemHints", "", 472),
    ("alpnasReadInstallerState", "", 485),
    ("alpnasTailLog", "", 508),
    ("alpnasFormatInstallerBytes", "", 530),
];

pub async fn alpnasinstallerserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerServiceInit" })
}

pub async fn alpnashandleinstallerstate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasHandleInstallerState" })
}

pub async fn alpnashandleinstallertargets(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasHandleInstallerTargets" })
}

pub async fn alpnashandleinstallerstart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasHandleInstallerStart" })
}

pub async fn alpnasinstalleravailable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerAvailable" })
}

pub async fn alpnasislivemode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasIsLiveMode" })
}

pub async fn alpnascollectinstallertargets(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasCollectInstallerTargets" })
}

pub async fn alpnasinstallerblockdiskcandidates(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerBlockDiskCandidates" })
}

pub async fn alpnasinstallerdevicesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerDeviceSize" })
}

pub async fn alpnasinstallerdiskisbootmedium(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerDiskIsBootMedium" })
}

pub async fn alpnasinstallerdevicename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerDeviceName" })
}

pub async fn alpnasinstallerreadsysfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerReadSysfs" })
}

pub async fn alpnasinstallersysfsbool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerSysfsBool" })
}

pub async fn alpnasinstallertransport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerTransport" })
}

pub async fn alpnasinstallerishotplug(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerIsHotplug" })
}

pub async fn alpnasinstallerpartitiondevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerPartitionDevices" })
}

pub async fn alpnasinstallerpartitioncount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerPartitionCount" })
}

pub async fn alpnasinstallermountedchildren(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerMountedChildren" })
}

pub async fn alpnasinstallerfilesystemhints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasInstallerFilesystemHints" })
}

pub async fn alpnasreadinstallerstate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasReadInstallerState" })
}

pub async fn alpnastaillog(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasTailLog" })
}

pub async fn alpnasformatinstallerbytes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "installer_alpnas.go", function: "alpnasFormatInstallerBytes" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
