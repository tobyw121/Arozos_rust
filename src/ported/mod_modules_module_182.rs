//! Original Go file: `mod/modules/module.go`
//! Package: `modules`; LOC: 255; SHA256: `054922e1fd95efb0d4496ada5556feb95afa610d68e0231e57fca4ff81d64bc1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/modules/module.go", package: "modules", go_loc: 255, functions: 11, types: 2, sha256: "054922e1fd95efb0d4496ada5556feb95afa610d68e0231e57fca4ff81d64bc1" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "sort",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ModuleInfo", "struct", 13),
    ("ModuleHandler", "struct", 32),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewModuleHandler", "", 38),
    ("RegisterModule", "m *ModuleHandler", 47),
    ("ModuleSortList", "m *ModuleHandler", 58),
    ("RegisterModuleFromJSON", "m *ModuleHandler", 65),
    ("RegisterModuleFromAGI", "m *ModuleHandler", 78),
    ("DeregisterModule", "m *ModuleHandler", 91),
    ("GetModuleNameList", "m *ModuleHandler", 103),
    ("HandleDefaultLauncher", "m *ModuleHandler", 112),
    ("ListLoadedModules", "m *ModuleHandler", 203),
    ("GetModuleInfoByID", "m *ModuleHandler", 219),
    ("GetLaunchParameter", "m *ModuleHandler", 228),
];

pub async fn newmodulehandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "NewModuleHandler" })
}

pub async fn modulehandler_registermodule(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.RegisterModule" })
}

pub async fn modulehandler_modulesortlist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.ModuleSortList" })
}

pub async fn modulehandler_registermodulefromjson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.RegisterModuleFromJSON" })
}

pub async fn modulehandler_registermodulefromagi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.RegisterModuleFromAGI" })
}

pub async fn modulehandler_deregistermodule(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.DeregisterModule" })
}

pub async fn modulehandler_getmodulenamelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.GetModuleNameList" })
}

pub async fn modulehandler_handledefaultlauncher(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.HandleDefaultLauncher" })
}

pub async fn modulehandler_listloadedmodules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.ListLoadedModules" })
}

pub async fn modulehandler_getmoduleinfobyid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.GetModuleInfoByID" })
}

pub async fn modulehandler_getlaunchparameter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/module.go", function: "ModuleHandler.GetLaunchParameter" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
