//! Original Go file: `mod/disk/raid/handler.go`
//! Package: `raid`; LOC: 681; SHA256: `bc2f9d93394de8eca0cd3686285911c6ba52756c8de09251a420d11405b690a6`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/handler.go", package: "raid", go_loc: 681, functions: 15, types: 0, sha256: "bc2f9d93394de8eca0cd3686285911c6ba52756c8de09251a420d11405b690a6" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/disk/diskfs",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "path/filepath",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleRemoveDiskFromRAIDVol", "m *Manager", 23),
    ("HandleAddDiskToRAIDVol", "m *Manager", 79),
    ("HandleMdadmFlushReload", "m *Manager", 151),
    ("HandleResolveDiskModelLabel", "m *Manager", 161),
    ("HandlListChildrenDeviceInfo", "m *Manager", 182),
    ("HandleListUsableDevices", "m *Manager", 222),
    ("HandleLoadArrayDetail", "m *Manager", 243),
    ("HandleFormatRaidDevice", "m *Manager", 272),
    ("HandleListRaidDevices", "m *Manager", 306),
    ("HandleCreateRAIDDevice", "m *Manager", 328),
    ("HandleRaidDevicesAssemble", "m *Manager", 462),
    ("HandleRemoveRaideDevice", "m *Manager", 473),
    ("HandleForceAssembleReload", "m *Manager", 575),
    ("HandleGrowRAIDArray", "m *Manager", 587),
    ("HandleRenderOverview", "m *Manager", 627),
];

pub async fn manager_handleremovediskfromraidvol(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleRemoveDiskFromRAIDVol" })
}

pub async fn manager_handleadddisktoraidvol(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleAddDiskToRAIDVol" })
}

pub async fn manager_handlemdadmflushreload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleMdadmFlushReload" })
}

pub async fn manager_handleresolvediskmodellabel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleResolveDiskModelLabel" })
}

pub async fn manager_handllistchildrendeviceinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandlListChildrenDeviceInfo" })
}

pub async fn manager_handlelistusabledevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleListUsableDevices" })
}

pub async fn manager_handleloadarraydetail(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleLoadArrayDetail" })
}

pub async fn manager_handleformatraiddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleFormatRaidDevice" })
}

pub async fn manager_handlelistraiddevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleListRaidDevices" })
}

pub async fn manager_handlecreateraiddevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleCreateRAIDDevice" })
}

pub async fn manager_handleraiddevicesassemble(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleRaidDevicesAssemble" })
}

pub async fn manager_handleremoveraidedevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleRemoveRaideDevice" })
}

pub async fn manager_handleforceassemblereload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleForceAssembleReload" })
}

pub async fn manager_handlegrowraidarray(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleGrowRAIDArray" })
}

pub async fn manager_handlerenderoverview(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/handler.go", function: "Manager.HandleRenderOverview" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
