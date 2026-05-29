//! Original Go file: `mod/filesystem/filesystem.go`
//! Package: `filesystem`; LOC: 499; SHA256: `bb49d591e6bdcb5eab2fa7746d102545f2bcec24ecef62880ed662325baa670b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/filesystem.go", package: "filesystem", go_loc: 499, functions: 14, types: 5, sha256: "bb49d591e6bdcb5eab2fa7746d102545f2bcec24ecef62880ed662325baa670b" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/md5",
    "encoding/hex",
    "errors",
    "fmt",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/filesystem/abstractions/ftpfs",
    "imuslab.com/arozos/mod/filesystem/abstractions/localfs",
    "imuslab.com/arozos/mod/filesystem/abstractions/sftpfs",
    "imuslab.com/arozos/mod/filesystem/abstractions/smbfs",
    "imuslab.com/arozos/mod/filesystem/abstractions/webdavfs",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/utils",
    "io",
    "io/fs",
    "log",
    "os",
    "path/filepath",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FileSystemOpeningOptions", "struct", 39),
    ("HierarchySpecificConfig", "interface", 57),
    ("FileSystemAbstraction", "interface", 59),
    ("RuntimePersistenceConfig", "struct", 94),
    ("FileSystemHandler", "struct", 99),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewFileSystemHandlersFromJSON", "", 117),
    ("NewFileSystemHandler", "", 139),
    ("IsNetworkDrive", "fsh *FileSystemHandler", 320),
    ("IsLocalDrive", "fsh *FileSystemHandler", 325),
    ("BufferRemoteToLocal", "fsh *FileSystemHandler", 340),
    ("IsRootOf", "fsh *FileSystemHandler", 386),
    ("GetUniquePathHash", "fsh *FileSystemHandler", 390),
    ("GetDirctorySizeFromRealPath", "fsh *FileSystemHandler", 407),
    ("GetDirctorySizeFromVpath", "fsh *FileSystemHandler", 437),
    ("ReloadFileSystelAbstraction", "fsh *FileSystemHandler", 443),
    ("RequierUserIsolation", "fsh *FileSystemHandler", 467),
    ("Close", "fsh *FileSystemHandler", 472),
    ("inSlice", "", 484),
    ("FileExists", "", 493),
];

pub async fn newfilesystemhandlersfromjson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "NewFileSystemHandlersFromJSON" })
}

pub async fn newfilesystemhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "NewFileSystemHandler" })
}

pub async fn filesystemhandler_isnetworkdrive(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.IsNetworkDrive" })
}

pub async fn filesystemhandler_islocaldrive(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.IsLocalDrive" })
}

pub async fn filesystemhandler_bufferremotetolocal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.BufferRemoteToLocal" })
}

pub async fn filesystemhandler_isrootof(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.IsRootOf" })
}

pub async fn filesystemhandler_getuniquepathhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.GetUniquePathHash" })
}

pub async fn filesystemhandler_getdirctorysizefromrealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.GetDirctorySizeFromRealPath" })
}

pub async fn filesystemhandler_getdirctorysizefromvpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.GetDirctorySizeFromVpath" })
}

pub async fn filesystemhandler_reloadfilesystelabstraction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.ReloadFileSystelAbstraction" })
}

pub async fn filesystemhandler_requieruserisolation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.RequierUserIsolation" })
}

pub async fn filesystemhandler_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileSystemHandler.Close" })
}

pub async fn inslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "inSlice" })
}

pub async fn fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/filesystem.go", function: "FileExists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
