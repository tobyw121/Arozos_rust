//! Original Go file: `mod/modules/installer.go`
//! Package: `modules`; LOC: 272; SHA256: `5715306b29bed71293924ed1fa1aca3689fc44ce531190d3d583d865d32ff01b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/modules/installer.go", package: "modules", go_loc: 272, functions: 6, types: 0, sha256: "5715306b29bed71293924ed1fa1aca3689fc44ce531190d3d583d865d32ff01b" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/go-git/go-git/v5",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/agi",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("InstallViaZip", "m *ModuleHandler", 29),
    ("ReloadAllModules", "m *ModuleHandler", 78),
    ("InstallModuleViaGit", "m *ModuleHandler", 95),
    ("ActivateModuleByRoot", "m *ModuleHandler", 150),
    ("HandleModuleInstallationListing", "m *ModuleHandler", 178),
    ("UninstallModule", "m *ModuleHandler", 237),
];

pub async fn modulehandler_installviazip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/installer.go", function: "ModuleHandler.InstallViaZip" })
}

pub async fn modulehandler_reloadallmodules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/installer.go", function: "ModuleHandler.ReloadAllModules" })
}

pub async fn modulehandler_installmoduleviagit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/installer.go", function: "ModuleHandler.InstallModuleViaGit" })
}

pub async fn modulehandler_activatemodulebyroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/installer.go", function: "ModuleHandler.ActivateModuleByRoot" })
}

pub async fn modulehandler_handlemoduleinstallationlisting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/installer.go", function: "ModuleHandler.HandleModuleInstallationListing" })
}

pub async fn modulehandler_uninstallmodule(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/modules/installer.go", function: "ModuleHandler.UninstallModule" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
