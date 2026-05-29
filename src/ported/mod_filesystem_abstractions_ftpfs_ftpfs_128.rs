//! Original Go file: `mod/filesystem/abstractions/ftpfs/ftpfs.go`
//! Package: `ftpfs`; LOC: 400; SHA256: `61ea29518347bac6e4d0f21c161feecc9a569ba921ceca855eace26d3b261ecf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/ftpfs/ftpfs.go", package: "ftpfs", go_loc: 400, functions: 32, types: 1, sha256: "61ea29518347bac6e4d0f21c161feecc9a569ba921ceca855eace26d3b261ecf" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "github.com/jlaffaye/ftp",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "log",
    "math/rand",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FTPFSAbstraction", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewFTPFSAbstraction", "", 35),
    ("makeConn", "l FTPFSAbstraction", 63),
    ("Chmod", "l FTPFSAbstraction", 105),
    ("Chown", "l FTPFSAbstraction", 108),
    ("Chtimes", "l FTPFSAbstraction", 111),
    ("Create", "l FTPFSAbstraction", 114),
    ("Mkdir", "l FTPFSAbstraction", 117),
    ("MkdirAll", "l FTPFSAbstraction", 132),
    ("Name", "l FTPFSAbstraction", 135),
    ("Open", "l FTPFSAbstraction", 138),
    ("OpenFile", "l FTPFSAbstraction", 141),
    ("Remove", "l FTPFSAbstraction", 144),
    ("RemoveAll", "l FTPFSAbstraction", 159),
    ("Rename", "l FTPFSAbstraction", 163),
    ("Stat", "l FTPFSAbstraction", 178),
    ("Close", "l FTPFSAbstraction", 181),
    ("VirtualPathToRealPath", "l FTPFSAbstraction", 190),
    ("RealPathToVirtualPath", "l FTPFSAbstraction", 194),
    ("FileExists", "l FTPFSAbstraction", 198),
    ("IsDir", "l FTPFSAbstraction", 209),
    ("Glob", "l FTPFSAbstraction", 224),
    ("GetFileSize", "l FTPFSAbstraction", 228),
    ("GetModTime", "l FTPFSAbstraction", 241),
    ("WriteFile", "l FTPFSAbstraction", 256),
    ("ReadFile", "l FTPFSAbstraction", 267),
    ("ReadDir", "l FTPFSAbstraction", 283),
    ("WriteStream", "l FTPFSAbstraction", 305),
    ("ReadStream", "l FTPFSAbstraction", 314),
    ("Walk", "l FTPFSAbstraction", 342),
    ("Heartbeat", "l FTPFSAbstraction", 359),
    ("filterFilepath", "", 364),
    ("walk", "l FTPFSAbstraction", 374),
];

pub async fn newftpfsabstraction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "NewFTPFSAbstraction" })
}

pub async fn ftpfsabstraction_makeconn(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.makeConn" })
}

pub async fn ftpfsabstraction_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Chmod" })
}

pub async fn ftpfsabstraction_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Chown" })
}

pub async fn ftpfsabstraction_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Chtimes" })
}

pub async fn ftpfsabstraction_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Create" })
}

pub async fn ftpfsabstraction_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Mkdir" })
}

pub async fn ftpfsabstraction_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.MkdirAll" })
}

pub async fn ftpfsabstraction_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Name" })
}

pub async fn ftpfsabstraction_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Open" })
}

pub async fn ftpfsabstraction_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.OpenFile" })
}

pub async fn ftpfsabstraction_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Remove" })
}

pub async fn ftpfsabstraction_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.RemoveAll" })
}

pub async fn ftpfsabstraction_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Rename" })
}

pub async fn ftpfsabstraction_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Stat" })
}

pub async fn ftpfsabstraction_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Close" })
}

pub async fn ftpfsabstraction_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.VirtualPathToRealPath" })
}

pub async fn ftpfsabstraction_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.RealPathToVirtualPath" })
}

pub async fn ftpfsabstraction_fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.FileExists" })
}

pub async fn ftpfsabstraction_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.IsDir" })
}

pub async fn ftpfsabstraction_glob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Glob" })
}

pub async fn ftpfsabstraction_getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.GetFileSize" })
}

pub async fn ftpfsabstraction_getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.GetModTime" })
}

pub async fn ftpfsabstraction_writefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.WriteFile" })
}

pub async fn ftpfsabstraction_readfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.ReadFile" })
}

pub async fn ftpfsabstraction_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.ReadDir" })
}

pub async fn ftpfsabstraction_writestream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.WriteStream" })
}

pub async fn ftpfsabstraction_readstream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.ReadStream" })
}

pub async fn ftpfsabstraction_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Walk" })
}

pub async fn ftpfsabstraction_heartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.Heartbeat" })
}

pub async fn filterfilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "filterFilepath" })
}

pub async fn ftpfsabstraction_walk_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpfs.go", function: "FTPFSAbstraction.walk" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
