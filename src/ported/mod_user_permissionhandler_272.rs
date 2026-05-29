//! Original Go file: `mod/user/permissionHandler.go`
//! Package: `user`; LOC: 225; SHA256: `8776e18fb6cb346430ad857e9e5390b36ebee5bf7a4faf53577e5cd9d27b2607`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/user/permissionHandler.go", package: "user", go_loc: 225, functions: 12, types: 0, sha256: "8776e18fb6cb346430ad857e9e5390b36ebee5bf7a4faf53577e5cd9d27b2607" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/storage",
    "imuslab.com/arozos/mod/utils",
    "log",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetModuleAccessPermission", "u *User", 16),
    ("GetUserAccessibleModules", "u *User", 39),
    ("IsAdmin", "u *User", 65),
    ("GetInterfaceModules", "u *User", 78),
    ("GetPathAccessPermission", "u *User", 96),
    ("CanRead", "u *User", 130),
    ("CanWrite", "u *User", 139),
    ("GetHighestAccessRightStoragePool", "u *User", 149),
    ("GetUserPermissionGroup", "u *User", 194),
    ("GetUserPermissionGroupNames", "u *User", 198),
    ("UserIsInOneOfTheGroupOf", "u *User", 207),
    ("SetUserPermissionGroup", "u *User", 219),
];

pub async fn user_getmoduleaccesspermission(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetModuleAccessPermission" })
}

pub async fn user_getuseraccessiblemodules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetUserAccessibleModules" })
}

pub async fn user_isadmin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.IsAdmin" })
}

pub async fn user_getinterfacemodules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetInterfaceModules" })
}

pub async fn user_getpathaccesspermission(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetPathAccessPermission" })
}

pub async fn user_canread(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.CanRead" })
}

pub async fn user_canwrite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.CanWrite" })
}

pub async fn user_gethighestaccessrightstoragepool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetHighestAccessRightStoragePool" })
}

pub async fn user_getuserpermissiongroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetUserPermissionGroup" })
}

pub async fn user_getuserpermissiongroupnames(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.GetUserPermissionGroupNames" })
}

pub async fn user_userisinoneofthegroupof(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.UserIsInOneOfTheGroupOf" })
}

pub async fn user_setuserpermissiongroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/permissionHandler.go", function: "User.SetUserPermissionGroup" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
