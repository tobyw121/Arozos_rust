//! Original Go file: `mod/network/webdav/webdav.go`
//! Package: `webdav`; LOC: 760; SHA256: `86daac1afefa7bac3b4ba4f4580d5a4e6a4b6c3fcbb152e46c437b49219a5b46`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/webdav.go", package: "webdav", go_loc: 760, functions: 18, types: 1, sha256: "86daac1afefa7bac3b4ba4f4580d5a4e6a4b6c3fcbb152e46c437b49219a5b46" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "golang.org/x/net/webdav",
    "io",
    "net/http",
    "net/url",
    "os",
    "path",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Handler", "struct", 20),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("stripPrefix", "h *Handler", 34),
    ("ServeHTTP", "h *Handler", 44),
    ("isWindowsClient", "", 88),
    ("lock", "h *Handler", 92),
    ("confirmLocks", "h *Handler", 107),
    ("handleOptions", "h *Handler", 185),
    ("handleGetHeadPost", "h *Handler", 207),
    ("handleDelete", "h *Handler", 236),
    ("handlePut", "h *Handler", 270),
    ("handleMkcol", "h *Handler", 317),
    ("handleCopyMove", "h *Handler", 346),
    ("handleLock", "h *Handler", 429),
    ("handleUnlock", "h *Handler", 523),
    ("handlePropfind", "h *Handler", 546),
    ("handleProppatch", "h *Handler", 626),
    ("makePropstatResponse", "", 669),
    ("parseDepth", "", 704),
    ("StatusText", "", 725),
];

pub async fn handler_stripprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.stripPrefix" })
}

pub async fn handler_servehttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.ServeHTTP" })
}

pub async fn iswindowsclient(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "isWindowsClient" })
}

pub async fn handler_lock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.lock" })
}

pub async fn handler_confirmlocks(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.confirmLocks" })
}

pub async fn handler_handleoptions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleOptions" })
}

pub async fn handler_handlegetheadpost(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleGetHeadPost" })
}

pub async fn handler_handledelete(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleDelete" })
}

pub async fn handler_handleput(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handlePut" })
}

pub async fn handler_handlemkcol(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleMkcol" })
}

pub async fn handler_handlecopymove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleCopyMove" })
}

pub async fn handler_handlelock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleLock" })
}

pub async fn handler_handleunlock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleUnlock" })
}

pub async fn handler_handlepropfind(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handlePropfind" })
}

pub async fn handler_handleproppatch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "Handler.handleProppatch" })
}

pub async fn makepropstatresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "makePropstatResponse" })
}

pub async fn parsedepth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "parseDepth" })
}

pub async fn statustext(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav.go", function: "StatusText" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
