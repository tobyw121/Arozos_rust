//! Original Go file: `mod/network/webdav/file.go`
//! Package: `webdav`; LOC: 802; SHA256: `146fead91d484cc0d40e5972d37fabfd8ba30dc7f21680efacb74488d8088f29`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/file.go", package: "webdav", go_loc: 802, functions: 36, types: 7, sha256: "146fead91d484cc0d40e5972d37fabfd8ba30dc7f21680efacb74488d8088f29" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "encoding/xml",
    "io",
    "net/http",
    "os",
    "path",
    "path/filepath",
    "runtime",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FileSystem", "interface", 40),
    ("File", "interface", 53),
    ("Dir", "s", 66),
    ("memFS", "struct", 150),
    ("memFSNode", "struct", 412),
    ("memFileInfo", "struct", 467),
    ("memFile", "struct", 484),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("slashClean", "", 23),
    ("resolve", "d Dir", 68),
    ("Mkdir", "d Dir", 81),
    ("OpenFile", "d Dir", 88),
    ("RemoveAll", "d Dir", 99),
    ("Rename", "d Dir", 110),
    ("Stat", "d Dir", 124),
    ("NewMemFS", "", 132),
    ("walk", "fs *memFS", 168),
    ("find", "fs *memFS", 229),
    ("Mkdir", "fs *memFS", 242),
    ("OpenFile", "fs *memFS", 265),
    ("RemoveAll", "fs *memFS", 325),
    ("Rename", "fs *memFS", 341),
    ("Stat", "fs *memFS", 392),
    ("stat", "n *memFSNode", 423),
    ("DeadProps", "n *memFSNode", 434),
    ("Patch", "n *memFSNode", 447),
    ("Name", "f *memFileInfo", 474),
    ("Size", "f *memFileInfo", 475),
    ("Mode", "f *memFileInfo", 476),
    ("ModTime", "f *memFileInfo", 477),
    ("IsDir", "f *memFileInfo", 478),
    ("Sys", "f *memFileInfo", 479),
    ("DeadProps", "f *memFile", 495),
    ("Patch", "f *memFile", 496),
    ("Close", "f *memFile", 498),
    ("Read", "f *memFile", 502),
    ("Readdir", "f *memFile", 516),
    ("Seek", "f *memFile", 543),
    ("Stat", "f *memFile", 565),
    ("Write", "f *memFile", 569),
    ("moveFiles", "", 610),
    ("copyProps", "", 637),
    ("copyFiles", "", 661),
    ("walkFS", "", 758),
];

pub async fn slashclean(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "slashClean" })
}

pub async fn dir_resolve(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "Dir.resolve" })
}

pub async fn dir_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "Dir.Mkdir" })
}

pub async fn dir_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "Dir.OpenFile" })
}

pub async fn dir_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "Dir.RemoveAll" })
}

pub async fn dir_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "Dir.Rename" })
}

pub async fn dir_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "Dir.Stat" })
}

pub async fn newmemfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "NewMemFS" })
}

pub async fn memfs_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.walk" })
}

pub async fn memfs_find(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.find" })
}

pub async fn memfs_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.Mkdir" })
}

pub async fn memfs_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.OpenFile" })
}

pub async fn memfs_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.RemoveAll" })
}

pub async fn memfs_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.Rename" })
}

pub async fn memfs_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFS.Stat" })
}

pub async fn memfsnode_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFSNode.stat" })
}

pub async fn memfsnode_deadprops(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFSNode.DeadProps" })
}

pub async fn memfsnode_patch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFSNode.Patch" })
}

pub async fn memfileinfo_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFileInfo.Name" })
}

pub async fn memfileinfo_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFileInfo.Size" })
}

pub async fn memfileinfo_mode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFileInfo.Mode" })
}

pub async fn memfileinfo_modtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFileInfo.ModTime" })
}

pub async fn memfileinfo_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFileInfo.IsDir" })
}

pub async fn memfileinfo_sys(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFileInfo.Sys" })
}

pub async fn memfile_deadprops(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.DeadProps" })
}

pub async fn memfile_patch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Patch" })
}

pub async fn memfile_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Close" })
}

pub async fn memfile_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Read" })
}

pub async fn memfile_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Readdir" })
}

pub async fn memfile_seek(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Seek" })
}

pub async fn memfile_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Stat" })
}

pub async fn memfile_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "memFile.Write" })
}

pub async fn movefiles(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "moveFiles" })
}

pub async fn copyprops(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "copyProps" })
}

pub async fn copyfiles(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "copyFiles" })
}

pub async fn walkfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file.go", function: "walkFS" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
