//! Original Go file: `mod/user/quota.go`
//! Package: `user`; LOC: 76; SHA256: `08f519b2d52bb2a7b5eaf463d614e246103ae16d503f0163f3718ee9485a7efb`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/user/quota.go", package: "user", go_loc: 76, functions: 5, types: 0, sha256: "08f519b2d52bb2a7b5eaf463d614e246103ae16d503f0163f3718ee9485a7efb" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem",
    "log",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HaveSpaceFor", "u *User", 19),
    ("SetOwnerOfFile", "u *User", 31),
    ("RemoveOwnershipFromFile", "u *User", 45),
    ("IsOwnerOfFile", "u *User", 59),
    ("GetFileOwner", "u *User", 69),
];

pub async fn user_havespacefor(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/quota.go", function: "User.HaveSpaceFor" })
}

pub async fn user_setowneroffile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/quota.go", function: "User.SetOwnerOfFile" })
}

pub async fn user_removeownershipfromfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/quota.go", function: "User.RemoveOwnershipFromFile" })
}

pub async fn user_isowneroffile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/quota.go", function: "User.IsOwnerOfFile" })
}

pub async fn user_getfileowner(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/quota.go", function: "User.GetFileOwner" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
