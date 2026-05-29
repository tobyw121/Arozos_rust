//! Original Go file: `mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go`
//! Package: `ftpfs`; LOC: 75; SHA256: `3b1f03051357e5b51c33dbba7162138c2c0ee9c748f47dc2729b16ad8fed12c3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", package: "ftpfs", go_loc: 75, functions: 12, types: 3, sha256: "3b1f03051357e5b51c33dbba7162138c2c0ee9c748f47dc2729b16ad8fed12c3" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/jlaffaye/ftp",
    "io/fs",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("File", "struct", 10),
    ("DirEntry", "struct", 14),
    ("FileInfo", "struct", 44),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("newDirEntryFromFTPEntry", "", 20),
    ("Name", "de DirEntry", 27),
    ("IsDir", "de DirEntry", 31),
    ("Type", "de DirEntry", 35),
    ("Info", "de DirEntry", 39),
    ("NewFileInfoFromEntry", "", 50),
    ("Name", "fi FileInfo", 58),
    ("Size", "fi FileInfo", 61),
    ("Mode", "fi FileInfo", 64),
    ("ModTime", "fi FileInfo", 67),
    ("IsDir", "fi FileInfo", 70),
    ("Sys", "fi FileInfo", 73),
];

pub async fn newdirentryfromftpentry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "newDirEntryFromFTPEntry" })
}

pub async fn direntry_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "DirEntry.Name" })
}

pub async fn direntry_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "DirEntry.IsDir" })
}

pub async fn direntry_type(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "DirEntry.Type" })
}

pub async fn direntry_info(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "DirEntry.Info" })
}

pub async fn newfileinfofromentry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "NewFileInfoFromEntry" })
}

pub async fn fileinfo_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "FileInfo.Name" })
}

pub async fn fileinfo_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "FileInfo.Size" })
}

pub async fn fileinfo_mode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "FileInfo.Mode" })
}

pub async fn fileinfo_modtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "FileInfo.ModTime" })
}

pub async fn fileinfo_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "FileInfo.IsDir" })
}

pub async fn fileinfo_sys(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/ftpfs/ftpFileWrapper.go", function: "FileInfo.Sys" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
