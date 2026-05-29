//! Original Go file: `mod/filesystem/abstractions/sftpfs/sftpfs.go`
//! Package: `sftpfs`; LOC: 338; SHA256: `ef36fea9e39e568e172b41388f245de7b2c7d889e2ad6186fcb7539edf88fb32`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/sftpfs/sftpfs.go", package: "sftpfs", go_loc: 338, functions: 29, types: 1, sha256: "ef36fea9e39e568e172b41388f245de7b2c7d889e2ad6186fcb7539edf88fb32" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "github.com/pkg/sftp",
    "golang.org/x/crypto/ssh",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "log",
    "net/url",
    "os",
    "path/filepath",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SFTPFileSystemAbstraction", "struct", 26),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSFTPFileSystemAbstraction", "", 40),
    ("Chmod", "s SFTPFileSystemAbstraction", 99),
    ("Chown", "s SFTPFileSystemAbstraction", 103),
    ("Chtimes", "s SFTPFileSystemAbstraction", 107),
    ("Create", "s SFTPFileSystemAbstraction", 111),
    ("Mkdir", "s SFTPFileSystemAbstraction", 116),
    ("MkdirAll", "s SFTPFileSystemAbstraction", 120),
    ("Name", "s SFTPFileSystemAbstraction", 124),
    ("Open", "s SFTPFileSystemAbstraction", 127),
    ("OpenFile", "s SFTPFileSystemAbstraction", 153),
    ("Remove", "s SFTPFileSystemAbstraction", 179),
    ("RemoveAll", "s SFTPFileSystemAbstraction", 183),
    ("Rename", "s SFTPFileSystemAbstraction", 191),
    ("Stat", "s SFTPFileSystemAbstraction", 196),
    ("Close", "s SFTPFileSystemAbstraction", 200),
    ("VirtualPathToRealPath", "s SFTPFileSystemAbstraction", 218),
    ("RealPathToVirtualPath", "s SFTPFileSystemAbstraction", 230),
    ("FileExists", "s SFTPFileSystemAbstraction", 238),
    ("IsDir", "s SFTPFileSystemAbstraction", 243),
    ("Glob", "s SFTPFileSystemAbstraction", 252),
    ("GetFileSize", "s SFTPFileSystemAbstraction", 257),
    ("GetModTime", "s SFTPFileSystemAbstraction", 266),
    ("WriteFile", "s SFTPFileSystemAbstraction", 275),
    ("ReadFile", "s SFTPFileSystemAbstraction", 285),
    ("ReadDir", "s SFTPFileSystemAbstraction", 294),
    ("WriteStream", "s SFTPFileSystemAbstraction", 309),
    ("ReadStream", "s SFTPFileSystemAbstraction", 318),
    ("Walk", "s SFTPFileSystemAbstraction", 327),
    ("Heartbeat", "s SFTPFileSystemAbstraction", 336),
];

pub async fn newsftpfilesystemabstraction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "NewSFTPFileSystemAbstraction" })
}

pub async fn sftpfilesystemabstraction_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Chmod" })
}

pub async fn sftpfilesystemabstraction_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Chown" })
}

pub async fn sftpfilesystemabstraction_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Chtimes" })
}

pub async fn sftpfilesystemabstraction_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Create" })
}

pub async fn sftpfilesystemabstraction_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Mkdir" })
}

pub async fn sftpfilesystemabstraction_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.MkdirAll" })
}

pub async fn sftpfilesystemabstraction_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Name" })
}

pub async fn sftpfilesystemabstraction_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Open" })
}

pub async fn sftpfilesystemabstraction_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.OpenFile" })
}

pub async fn sftpfilesystemabstraction_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Remove" })
}

pub async fn sftpfilesystemabstraction_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.RemoveAll" })
}

pub async fn sftpfilesystemabstraction_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Rename" })
}

pub async fn sftpfilesystemabstraction_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Stat" })
}

pub async fn sftpfilesystemabstraction_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Close" })
}

pub async fn sftpfilesystemabstraction_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.VirtualPathToRealPath" })
}

pub async fn sftpfilesystemabstraction_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.RealPathToVirtualPath" })
}

pub async fn sftpfilesystemabstraction_fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.FileExists" })
}

pub async fn sftpfilesystemabstraction_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.IsDir" })
}

pub async fn sftpfilesystemabstraction_glob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Glob" })
}

pub async fn sftpfilesystemabstraction_getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.GetFileSize" })
}

pub async fn sftpfilesystemabstraction_getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.GetModTime" })
}

pub async fn sftpfilesystemabstraction_writefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.WriteFile" })
}

pub async fn sftpfilesystemabstraction_readfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.ReadFile" })
}

pub async fn sftpfilesystemabstraction_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.ReadDir" })
}

pub async fn sftpfilesystemabstraction_writestream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.WriteStream" })
}

pub async fn sftpfilesystemabstraction_readstream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.ReadStream" })
}

pub async fn sftpfilesystemabstraction_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Walk" })
}

pub async fn sftpfilesystemabstraction_heartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpfs.go", function: "SFTPFileSystemAbstraction.Heartbeat" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
