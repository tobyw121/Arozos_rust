//! Original Go file: `mod/filesystem/abstractions/localfs/localfs.go`
//! Package: `localfs`; LOC: 189; SHA256: `79a1d21099a151fe0f8717f86e5556a4991673e6511450e6979a30593b2bf50d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/localfs/localfs.go", package: "localfs", go_loc: 189, functions: 29, types: 1, sha256: "79a1d21099a151fe0f8717f86e5556a4991673e6511450e6979a30593b2bf50d" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/utils",
    "io",
    "io/fs",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("LocalFileSystemAbstraction", "struct", 22),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewLocalFileSystemAbstraction", "", 29),
    ("Chmod", "l LocalFileSystemAbstraction", 38),
    ("Chown", "l LocalFileSystemAbstraction", 41),
    ("Chtimes", "l LocalFileSystemAbstraction", 44),
    ("Create", "l LocalFileSystemAbstraction", 47),
    ("Mkdir", "l LocalFileSystemAbstraction", 50),
    ("MkdirAll", "l LocalFileSystemAbstraction", 53),
    ("Name", "l LocalFileSystemAbstraction", 56),
    ("Open", "l LocalFileSystemAbstraction", 59),
    ("OpenFile", "l LocalFileSystemAbstraction", 62),
    ("Remove", "l LocalFileSystemAbstraction", 65),
    ("RemoveAll", "l LocalFileSystemAbstraction", 68),
    ("Rename", "l LocalFileSystemAbstraction", 71),
    ("Stat", "l LocalFileSystemAbstraction", 74),
    ("Close", "l LocalFileSystemAbstraction", 77),
    ("VirtualPathToRealPath", "l LocalFileSystemAbstraction", 85),
    ("RealPathToVirtualPath", "l LocalFileSystemAbstraction", 98),
    ("FileExists", "l LocalFileSystemAbstraction", 110),
    ("IsDir", "l LocalFileSystemAbstraction", 114),
    ("Glob", "l LocalFileSystemAbstraction", 131),
    ("GetFileSize", "l LocalFileSystemAbstraction", 135),
    ("GetModTime", "l LocalFileSystemAbstraction", 144),
    ("WriteFile", "l LocalFileSystemAbstraction", 157),
    ("ReadFile", "l LocalFileSystemAbstraction", 160),
    ("ReadDir", "l LocalFileSystemAbstraction", 163),
    ("WriteStream", "l LocalFileSystemAbstraction", 166),
    ("ReadStream", "l LocalFileSystemAbstraction", 175),
    ("Walk", "l LocalFileSystemAbstraction", 183),
    ("Heartbeat", "l LocalFileSystemAbstraction", 187),
];

pub async fn newlocalfilesystemabstraction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "NewLocalFileSystemAbstraction" })
}

pub async fn localfilesystemabstraction_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Chmod" })
}

pub async fn localfilesystemabstraction_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Chown" })
}

pub async fn localfilesystemabstraction_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Chtimes" })
}

pub async fn localfilesystemabstraction_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Create" })
}

pub async fn localfilesystemabstraction_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Mkdir" })
}

pub async fn localfilesystemabstraction_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.MkdirAll" })
}

pub async fn localfilesystemabstraction_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Name" })
}

pub async fn localfilesystemabstraction_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Open" })
}

pub async fn localfilesystemabstraction_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.OpenFile" })
}

pub async fn localfilesystemabstraction_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Remove" })
}

pub async fn localfilesystemabstraction_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.RemoveAll" })
}

pub async fn localfilesystemabstraction_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Rename" })
}

pub async fn localfilesystemabstraction_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Stat" })
}

pub async fn localfilesystemabstraction_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Close" })
}

pub async fn localfilesystemabstraction_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.VirtualPathToRealPath" })
}

pub async fn localfilesystemabstraction_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.RealPathToVirtualPath" })
}

pub async fn localfilesystemabstraction_fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.FileExists" })
}

pub async fn localfilesystemabstraction_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.IsDir" })
}

pub async fn localfilesystemabstraction_glob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Glob" })
}

pub async fn localfilesystemabstraction_getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.GetFileSize" })
}

pub async fn localfilesystemabstraction_getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.GetModTime" })
}

pub async fn localfilesystemabstraction_writefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.WriteFile" })
}

pub async fn localfilesystemabstraction_readfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.ReadFile" })
}

pub async fn localfilesystemabstraction_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.ReadDir" })
}

pub async fn localfilesystemabstraction_writestream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.WriteStream" })
}

pub async fn localfilesystemabstraction_readstream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.ReadStream" })
}

pub async fn localfilesystemabstraction_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Walk" })
}

pub async fn localfilesystemabstraction_heartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/localfs/localfs.go", function: "LocalFileSystemAbstraction.Heartbeat" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
