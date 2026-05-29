//! Original Go file: `mod/permission/request.go`
//! Package: `permission`; LOC: 242; SHA256: `2d3712866a4df55b612994b8aba7fe781b0827b9c15dce0e73556141164db32f`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/permission/request.go", package: "permission", go_loc: 242, functions: 4, types: 0, sha256: "2d3712866a4df55b612994b8aba7fe781b0827b9c15dce0e73556141164db32f" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleListGroup", "h *PermissionHandler", 23),
    ("HandleGroupEdit", "h *PermissionHandler", 49),
    ("HandleGroupCreate", "h *PermissionHandler", 130),
    ("HandleGroupRemove", "h *PermissionHandler", 203),
];

pub async fn permissionhandler_handlelistgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/request.go", function: "PermissionHandler.HandleListGroup" })
}

pub async fn permissionhandler_handlegroupedit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/request.go", function: "PermissionHandler.HandleGroupEdit" })
}

pub async fn permissionhandler_handlegroupcreate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/request.go", function: "PermissionHandler.HandleGroupCreate" })
}

pub async fn permissionhandler_handlegroupremove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/request.go", function: "PermissionHandler.HandleGroupRemove" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
