//! Original Go file: `mod/storage/webdav/webdav.go`
//! Package: `webdav`; LOC: 334; SHA256: `96df6c8b83c7ea2a727a145b34a7cf09e6d979a2c57394cd0bae4ee7420dbe3d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/webdav/webdav.go", package: "webdav", go_loc: 334, functions: 7, types: 2, sha256: "96df6c8b83c7ea2a727a145b34a7cf09e6d979a2c57394cd0bae4ee7420dbe3d" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/hidden",
    "imuslab.com/arozos/mod/filesystem/metadata",
    "imuslab.com/arozos/mod/network/webdav",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "sort",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Server", "struct", 30),
    ("WindowClientInfo", "struct", 44),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewServer", "", 53),
    ("HandleClearAllPending", "s *Server", 74),
    ("HandlePermissionEdit", "s *Server", 91),
    ("HandleConnectionList", "s *Server", 150),
    ("HandleRequest", "s *Server", 196),
    ("serveReadOnlyWebDav", "s *Server", 300),
    ("getFsFromRealRoot", "s *Server", 311),
];

pub async fn newserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "NewServer" })
}

pub async fn server_handleclearallpending(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "Server.HandleClearAllPending" })
}

pub async fn server_handlepermissionedit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "Server.HandlePermissionEdit" })
}

pub async fn server_handleconnectionlist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "Server.HandleConnectionList" })
}

pub async fn server_handlerequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "Server.HandleRequest" })
}

pub async fn server_servereadonlywebdav(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "Server.serveReadOnlyWebDav" })
}

pub async fn server_getfsfromrealroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/webdav.go", function: "Server.getFsFromRealRoot" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
