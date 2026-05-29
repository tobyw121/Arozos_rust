//! Original Go file: `mod/permission/group.go`
//! Package: `permission`; LOC: 37; SHA256: `5b03d368fb5e704ffeae33b5d203118171e04e634f7e61388045c420e68a35d6`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/permission/group.go", package: "permission", go_loc: 37, functions: 3, types: 0, sha256: "5b03d368fb5e704ffeae33b5d203118171e04e634f7e61388045c420e68a35d6" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/utils",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("AddModule", "gp *PermissionGroup", 5),
    ("RemoveModule", "gp *PermissionGroup", 11),
    ("Remove", "gp *PermissionGroup", 25),
];

pub async fn permissiongroup_addmodule(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/group.go", function: "PermissionGroup.AddModule" })
}

pub async fn permissiongroup_removemodule(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/group.go", function: "PermissionGroup.RemoveModule" })
}

pub async fn permissiongroup_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/group.go", function: "PermissionGroup.Remove" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
