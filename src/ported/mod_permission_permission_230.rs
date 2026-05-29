//! Original Go file: `mod/permission/permission.go`
//! Package: `permission`; LOC: 241; SHA256: `618b64362c278aa85a677e1ae461d524e4f18d597f726e3cf7cc0c0a94f6e800`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/permission/permission.go", package: "permission", go_loc: 241, functions: 11, types: 2, sha256: "618b64362c278aa85a677e1ae461d524e4f18d597f726e3cf7cc0c0a94f6e800" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/storage",
    "imuslab.com/arozos/mod/utils",
    "log",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("PermissionGroup", "struct", 15),
    ("PermissionHandler", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("normalizeGroupName", "", 30),
    ("IsAdministratorGroupName", "", 34),
    ("isAdministratorGroupName", "", 43),
    ("NewPermissionHandler", "", 47),
    ("GroupExists", "h *PermissionHandler", 67),
    ("LoadPermissionGroupsFromDatabase", "h *PermissionHandler", 78),
    ("GetUsersPermissionGroup", "h *PermissionHandler", 123),
    ("UpdatePermissionGroup", "h *PermissionHandler", 153),
    ("NewPermissionGroup", "h *PermissionHandler", 186),
    ("GetPermissionGroupByNameList", "h *PermissionHandler", 222),
    ("GetPermissionGroupByName", "h *PermissionHandler", 234),
];

pub async fn normalizegroupname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "normalizeGroupName" })
}

pub async fn isadministratorgroupname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "IsAdministratorGroupName" })
}

pub async fn isadministratorgroupname_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "isAdministratorGroupName" })
}

pub async fn newpermissionhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "NewPermissionHandler" })
}

pub async fn permissionhandler_groupexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.GroupExists" })
}

pub async fn permissionhandler_loadpermissiongroupsfromdatabase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.LoadPermissionGroupsFromDatabase" })
}

pub async fn permissionhandler_getuserspermissiongroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.GetUsersPermissionGroup" })
}

pub async fn permissionhandler_updatepermissiongroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.UpdatePermissionGroup" })
}

pub async fn permissionhandler_newpermissiongroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.NewPermissionGroup" })
}

pub async fn permissionhandler_getpermissiongroupbynamelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.GetPermissionGroupByNameList" })
}

pub async fn permissionhandler_getpermissiongroupbyname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/permission.go", function: "PermissionHandler.GetPermissionGroupByName" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
