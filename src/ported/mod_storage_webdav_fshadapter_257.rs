//! Original Go file: `mod/storage/webdav/fshAdapter.go`
//! Package: `webdav`; LOC: 188; SHA256: `7219faf5c88cce8bb9b31488f7731b7eca31c82729ff167f1afc36a03eeea13e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/webdav/fshAdapter.go", package: "webdav", go_loc: 188, functions: 14, types: 2, sha256: "7219faf5c88cce8bb9b31488f7731b7eca31c82729ff167f1afc36a03eeea13e" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "context",
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/network/webdav",
    "io/fs",
    "log",
    "os",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FshWebDAVAdapter", "struct", 16),
    ("BufferFsIoHandler", "struct", 21),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Close", "b BufferFsIoHandler", 28),
    ("newBufferFsIoHandler", "", 32),
    ("Read", "b BufferFsIoHandler", 67),
    ("Seek", "b BufferFsIoHandler", 72),
    ("Readdir", "b BufferFsIoHandler", 77),
    ("Stat", "b BufferFsIoHandler", 100),
    ("Write", "b BufferFsIoHandler", 105),
    ("NewFshWebDAVAdapter", "", 115),
    ("requestPathToRealPath", "a *FshWebDAVAdapter", 122),
    ("Mkdir", "a *FshWebDAVAdapter", 136),
    ("OpenFile", "a *FshWebDAVAdapter", 143),
    ("RemoveAll", "a *FshWebDAVAdapter", 159),
    ("Rename", "a *FshWebDAVAdapter", 166),
    ("Stat", "a *FshWebDAVAdapter", 179),
];

pub async fn bufferfsiohandler_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "BufferFsIoHandler.Close" })
}

pub async fn newbufferfsiohandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "newBufferFsIoHandler" })
}

pub async fn bufferfsiohandler_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "BufferFsIoHandler.Read" })
}

pub async fn bufferfsiohandler_seek(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "BufferFsIoHandler.Seek" })
}

pub async fn bufferfsiohandler_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "BufferFsIoHandler.Readdir" })
}

pub async fn bufferfsiohandler_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "BufferFsIoHandler.Stat" })
}

pub async fn bufferfsiohandler_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "BufferFsIoHandler.Write" })
}

pub async fn newfshwebdavadapter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "NewFshWebDAVAdapter" })
}

pub async fn fshwebdavadapter_requestpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "FshWebDAVAdapter.requestPathToRealPath" })
}

pub async fn fshwebdavadapter_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "FshWebDAVAdapter.Mkdir" })
}

pub async fn fshwebdavadapter_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "FshWebDAVAdapter.OpenFile" })
}

pub async fn fshwebdavadapter_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "FshWebDAVAdapter.RemoveAll" })
}

pub async fn fshwebdavadapter_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "FshWebDAVAdapter.Rename" })
}

pub async fn fshwebdavadapter_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/fshAdapter.go", function: "FshWebDAVAdapter.Stat" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
