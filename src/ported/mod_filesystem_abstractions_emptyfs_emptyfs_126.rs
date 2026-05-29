//! Original Go file: `mod/filesystem/abstractions/emptyfs/emptyfs.go`
//! Package: `emptyfs`; LOC: 124; SHA256: `b2f3b3b52a04101dcb9cec52a97c93eb2af2e2de093e4ebb5cef14f1077a0607`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/emptyfs/emptyfs.go", package: "emptyfs", go_loc: 124, functions: 29, types: 1, sha256: "b2f3b3b52a04101dcb9cec52a97c93eb2af2e2de093e4ebb5cef14f1077a0607" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "os",
    "path/filepath",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("EmptyFileSystemAbstraction", "struct", 20),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewEmptyFileSystemAbstraction", "", 23),
    ("Chmod", "l EmptyFileSystemAbstraction", 27),
    ("Chown", "l EmptyFileSystemAbstraction", 30),
    ("Chtimes", "l EmptyFileSystemAbstraction", 33),
    ("Create", "l EmptyFileSystemAbstraction", 36),
    ("Mkdir", "l EmptyFileSystemAbstraction", 39),
    ("MkdirAll", "l EmptyFileSystemAbstraction", 42),
    ("Name", "l EmptyFileSystemAbstraction", 45),
    ("Open", "l EmptyFileSystemAbstraction", 48),
    ("OpenFile", "l EmptyFileSystemAbstraction", 51),
    ("Remove", "l EmptyFileSystemAbstraction", 54),
    ("RemoveAll", "l EmptyFileSystemAbstraction", 57),
    ("Rename", "l EmptyFileSystemAbstraction", 60),
    ("Stat", "l EmptyFileSystemAbstraction", 63),
    ("Close", "l EmptyFileSystemAbstraction", 66),
    ("VirtualPathToRealPath", "l EmptyFileSystemAbstraction", 74),
    ("RealPathToVirtualPath", "l EmptyFileSystemAbstraction", 78),
    ("FileExists", "l EmptyFileSystemAbstraction", 82),
    ("IsDir", "l EmptyFileSystemAbstraction", 86),
    ("Glob", "l EmptyFileSystemAbstraction", 90),
    ("GetFileSize", "l EmptyFileSystemAbstraction", 94),
    ("GetModTime", "l EmptyFileSystemAbstraction", 98),
    ("WriteFile", "l EmptyFileSystemAbstraction", 102),
    ("ReadFile", "l EmptyFileSystemAbstraction", 105),
    ("ReadDir", "l EmptyFileSystemAbstraction", 108),
    ("WriteStream", "l EmptyFileSystemAbstraction", 111),
    ("ReadStream", "l EmptyFileSystemAbstraction", 114),
    ("Walk", "l EmptyFileSystemAbstraction", 118),
    ("Heartbeat", "l EmptyFileSystemAbstraction", 122),
];

pub async fn newemptyfilesystemabstraction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "NewEmptyFileSystemAbstraction" })
}

pub async fn emptyfilesystemabstraction_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Chmod" })
}

pub async fn emptyfilesystemabstraction_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Chown" })
}

pub async fn emptyfilesystemabstraction_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Chtimes" })
}

pub async fn emptyfilesystemabstraction_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Create" })
}

pub async fn emptyfilesystemabstraction_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Mkdir" })
}

pub async fn emptyfilesystemabstraction_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.MkdirAll" })
}

pub async fn emptyfilesystemabstraction_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Name" })
}

pub async fn emptyfilesystemabstraction_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Open" })
}

pub async fn emptyfilesystemabstraction_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.OpenFile" })
}

pub async fn emptyfilesystemabstraction_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Remove" })
}

pub async fn emptyfilesystemabstraction_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.RemoveAll" })
}

pub async fn emptyfilesystemabstraction_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Rename" })
}

pub async fn emptyfilesystemabstraction_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Stat" })
}

pub async fn emptyfilesystemabstraction_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Close" })
}

pub async fn emptyfilesystemabstraction_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.VirtualPathToRealPath" })
}

pub async fn emptyfilesystemabstraction_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.RealPathToVirtualPath" })
}

pub async fn emptyfilesystemabstraction_fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.FileExists" })
}

pub async fn emptyfilesystemabstraction_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.IsDir" })
}

pub async fn emptyfilesystemabstraction_glob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Glob" })
}

pub async fn emptyfilesystemabstraction_getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.GetFileSize" })
}

pub async fn emptyfilesystemabstraction_getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.GetModTime" })
}

pub async fn emptyfilesystemabstraction_writefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.WriteFile" })
}

pub async fn emptyfilesystemabstraction_readfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.ReadFile" })
}

pub async fn emptyfilesystemabstraction_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.ReadDir" })
}

pub async fn emptyfilesystemabstraction_writestream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.WriteStream" })
}

pub async fn emptyfilesystemabstraction_readstream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.ReadStream" })
}

pub async fn emptyfilesystemabstraction_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Walk" })
}

pub async fn emptyfilesystemabstraction_heartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/emptyfs/emptyfs.go", function: "EmptyFileSystemAbstraction.Heartbeat" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
