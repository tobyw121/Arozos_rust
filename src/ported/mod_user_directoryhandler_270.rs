//! Original Go file: `mod/user/directoryHandler.go`
//! Package: `user`; LOC: 257; SHA256: `1d9e94f4be584a70964f9616a3d2ab7b9000c3cec442691e394d2a23e04c9c2c`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/user/directoryHandler.go", package: "user", go_loc: 257, functions: 9, types: 0, sha256: "1d9e94f4be584a70964f9616a3d2ab7b9000c3cec442691e394d2a23e04c9c2c" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetHomeDirectory", "u *User", 9),
    ("GetHomeFileSystemHandler", "u *User", 25),
    ("GetAllAccessibleFileSystemHandler", "u *User", 30),
    ("GetRootFSHFromVpathInUserScope", "u *User", 43),
    ("GetAllFileSystemHandler", "u *User", 55),
    ("VirtualPathToRealPath", "u *User", 88),
    ("RealPathToVirtualPath", "u *User", 144),
    ("GetFileSystemHandlerFromVirtualPath", "u *User", 242),
    ("GetFileSystemHandlerFromRealPath", "u *User", 249),
];

pub async fn user_gethomedirectory(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetHomeDirectory" })
}

pub async fn user_gethomefilesystemhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetHomeFileSystemHandler" })
}

pub async fn user_getallaccessiblefilesystemhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetAllAccessibleFileSystemHandler" })
}

pub async fn user_getrootfshfromvpathinuserscope(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetRootFSHFromVpathInUserScope" })
}

pub async fn user_getallfilesystemhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetAllFileSystemHandler" })
}

pub async fn user_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.VirtualPathToRealPath" })
}

pub async fn user_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.RealPathToVirtualPath" })
}

pub async fn user_getfilesystemhandlerfromvirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetFileSystemHandlerFromVirtualPath" })
}

pub async fn user_getfilesystemhandlerfromrealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/directoryHandler.go", function: "User.GetFileSystemHandlerFromRealPath" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
