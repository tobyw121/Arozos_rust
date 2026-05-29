//! Original Go file: `mod/user/user.go`
//! Package: `user`; LOC: 185; SHA256: `0e68ed32277af6535433c67131bf37d214a00b0bc027043f1a14f41a4b581246`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/user/user.go", package: "user", go_loc: 185, functions: 9, types: 2, sha256: "0e68ed32277af6535433c67131bf37d214a00b0bc027043f1a14f41a4b581246" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "golang.org/x/sync/syncmap",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/quota",
    "imuslab.com/arozos/mod/share/shareEntry",
    "imuslab.com/arozos/mod/storage",
    "log",
    "net/http",
    "os",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("User", "struct", 25),
    ("UserHandler", "struct", 34),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewUserHandler", "", 45),
    ("GetAuthAgent", "u *UserHandler", 56),
    ("GetPermissionHandler", "u *UserHandler", 60),
    ("GetStoragePool", "u *UserHandler", 65),
    ("GetDatabase", "u *UserHandler", 69),
    ("UpdateStoragePool", "u *UserHandler", 73),
    ("GetUserInfoFromUsername", "u *UserHandler", 78),
    ("GetUserInfoFromRequest", "u *UserHandler", 149),
    ("GetUsersInPermissionGroup", "u *UserHandler", 163),
];

pub async fn newuserhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "NewUserHandler" })
}

pub async fn userhandler_getauthagent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetAuthAgent" })
}

pub async fn userhandler_getpermissionhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetPermissionHandler" })
}

pub async fn userhandler_getstoragepool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetStoragePool" })
}

pub async fn userhandler_getdatabase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetDatabase" })
}

pub async fn userhandler_updatestoragepool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.UpdateStoragePool" })
}

pub async fn userhandler_getuserinfofromusername(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetUserInfoFromUsername" })
}

pub async fn userhandler_getuserinfofromrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetUserInfoFromRequest" })
}

pub async fn userhandler_getusersinpermissiongroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/user.go", function: "UserHandler.GetUsersInPermissionGroup" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
