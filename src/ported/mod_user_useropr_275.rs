//! Original Go file: `mod/user/useropr.go`
//! Package: `user`; LOC: 30; SHA256: `77924c4660027b338c9cfe4e832282600bc4913d96d004fd76c25c532ff8cf14`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/user/useropr.go", package: "user", go_loc: 30, functions: 4, types: 0, sha256: "77924c4660027b338c9cfe4e832282600bc4913d96d004fd76c25c532ff8cf14" };

pub const GO_IMPORTS: &[&str] = &[
    "log",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Parent", "u *User", 6),
    ("RemoveUser", "u *User", 11),
    ("GetUserIcon", "u *User", 21),
    ("SetUserIcon", "u *User", 28),
];

pub async fn user_parent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/useropr.go", function: "User.Parent" })
}

pub async fn user_removeuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/useropr.go", function: "User.RemoveUser" })
}

pub async fn user_getusericon(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/useropr.go", function: "User.GetUserIcon" })
}

pub async fn user_setusericon(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/useropr.go", function: "User.SetUserIcon" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
