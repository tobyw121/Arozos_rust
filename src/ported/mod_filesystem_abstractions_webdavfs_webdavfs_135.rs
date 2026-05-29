//! Original Go file: `mod/filesystem/abstractions/webdavfs/webdavfs.go`
//! Package: `webdavfs`; LOC: 371; SHA256: `80b23362e7c7892bc2fd538b5427ad913c2048a3fee44acdb5ae11762d808bf0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/webdavfs/webdavfs.go", package: "webdavfs", go_loc: 371, functions: 33, types: 1, sha256: "80b23362e7c7892bc2fd538b5427ad913c2048a3fee44acdb5ae11762d808bf0" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/studio-b12/gowebdav",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "log",
    "os",
    "path/filepath",
    "regexp",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("WebDAVFileSystem", "struct", 27),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewWebDAVMount", "", 35),
    ("Chmod", "e WebDAVFileSystem", 54),
    ("Chown", "e WebDAVFileSystem", 57),
    ("Chtimes", "e WebDAVFileSystem", 60),
    ("Create", "e WebDAVFileSystem", 63),
    ("Mkdir", "e WebDAVFileSystem", 66),
    ("MkdirAll", "e WebDAVFileSystem", 70),
    ("Name", "e WebDAVFileSystem", 74),
    ("Open", "e WebDAVFileSystem", 77),
    ("OpenFile", "e WebDAVFileSystem", 80),
    ("Remove", "e WebDAVFileSystem", 87),
    ("RemoveAll", "e WebDAVFileSystem", 91),
    ("Rename", "e WebDAVFileSystem", 95),
    ("Stat", "e WebDAVFileSystem", 115),
    ("VirtualPathToRealPath", "e WebDAVFileSystem", 120),
    ("RealPathToVirtualPath", "e WebDAVFileSystem", 135),
    ("FileExists", "e WebDAVFileSystem", 146),
    ("IsDir", "e WebDAVFileSystem", 155),
    ("Glob", "e WebDAVFileSystem", 165),
    ("GetFileSize", "e WebDAVFileSystem", 177),
    ("GetModTime", "e WebDAVFileSystem", 188),
    ("WriteFile", "e WebDAVFileSystem", 198),
    ("ReadFile", "e WebDAVFileSystem", 203),
    ("ReadDir", "e WebDAVFileSystem", 212),
    ("WriteStream", "e WebDAVFileSystem", 225),
    ("ReadStream", "e WebDAVFileSystem", 230),
    ("Walk", "e WebDAVFileSystem", 235),
    ("Close", "e WebDAVFileSystem", 245),
    ("Heartbeat", "e WebDAVFileSystem", 250),
    ("walk", "e WebDAVFileSystem", 259),
    ("globpath", "e WebDAVFileSystem", 286),
    ("filterFilepath", "", 348),
    ("wildCardToRegexp", "", 358),
];

pub async fn newwebdavmount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "NewWebDAVMount" })
}

pub async fn webdavfilesystem_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Chmod" })
}

pub async fn webdavfilesystem_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Chown" })
}

pub async fn webdavfilesystem_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Chtimes" })
}

pub async fn webdavfilesystem_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Create" })
}

pub async fn webdavfilesystem_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Mkdir" })
}

pub async fn webdavfilesystem_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.MkdirAll" })
}

pub async fn webdavfilesystem_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Name" })
}

pub async fn webdavfilesystem_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Open" })
}

pub async fn webdavfilesystem_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.OpenFile" })
}

pub async fn webdavfilesystem_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Remove" })
}

pub async fn webdavfilesystem_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.RemoveAll" })
}

pub async fn webdavfilesystem_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Rename" })
}

pub async fn webdavfilesystem_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Stat" })
}

pub async fn webdavfilesystem_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.VirtualPathToRealPath" })
}

pub async fn webdavfilesystem_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.RealPathToVirtualPath" })
}

pub async fn webdavfilesystem_fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.FileExists" })
}

pub async fn webdavfilesystem_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.IsDir" })
}

pub async fn webdavfilesystem_glob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Glob" })
}

pub async fn webdavfilesystem_getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.GetFileSize" })
}

pub async fn webdavfilesystem_getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.GetModTime" })
}

pub async fn webdavfilesystem_writefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.WriteFile" })
}

pub async fn webdavfilesystem_readfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.ReadFile" })
}

pub async fn webdavfilesystem_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.ReadDir" })
}

pub async fn webdavfilesystem_writestream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.WriteStream" })
}

pub async fn webdavfilesystem_readstream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.ReadStream" })
}

pub async fn webdavfilesystem_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Walk" })
}

pub async fn webdavfilesystem_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Close" })
}

pub async fn webdavfilesystem_heartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.Heartbeat" })
}

pub async fn webdavfilesystem_walk_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.walk" })
}

pub async fn webdavfilesystem_globpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "WebDAVFileSystem.globpath" })
}

pub async fn filterfilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "filterFilepath" })
}

pub async fn wildcardtoregexp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/webdavfs/webdavfs.go", function: "wildCardToRegexp" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
