//! Original Go file: `mod/network/webdav/prop.go`
//! Package: `webdav`; LOC: 469; SHA256: `63231be3d3b879e4d98a855270c3c6b0d0e9d1934887ed940a108af9e3932948`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/prop.go", package: "webdav", go_loc: 469, functions: 13, types: 5, sha256: "63231be3d3b879e4d98a855270c3c6b0d0e9d1934887ed940a108af9e3932948" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "context",
    "encoding/xml",
    "errors",
    "fmt",
    "io",
    "mime",
    "net/http",
    "os",
    "path/filepath",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Proppatch", "struct", 23),
    ("Propstat", "struct", 33),
    ("DeadPropsHolder", "interface", 84),
    ("ContentTyper", "interface", 394),
    ("ETager", "interface", 440),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("makePropstats", "", 57),
    ("props", "", 169),
    ("propnames", "", 217),
    ("allprop", "", 257),
    ("patch", "", 277),
    ("escapeXML", "", 339),
    ("findResourceType", "", 359),
    ("findDisplayName", "", 366),
    ("findContentLength", "", 374),
    ("findLastModified", "", 378),
    ("findContentType", "", 403),
    ("findETag", "", 450),
    ("findSupportedLock", "", 463),
];

pub async fn makepropstats(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "makePropstats" })
}

pub async fn props(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "props" })
}

pub async fn propnames(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "propnames" })
}

pub async fn allprop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "allprop" })
}

pub async fn patch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "patch" })
}

pub async fn escapexml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "escapeXML" })
}

pub async fn findresourcetype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findResourceType" })
}

pub async fn finddisplayname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findDisplayName" })
}

pub async fn findcontentlength(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findContentLength" })
}

pub async fn findlastmodified(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findLastModified" })
}

pub async fn findcontenttype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findContentType" })
}

pub async fn findetag(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findETag" })
}

pub async fn findsupportedlock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop.go", function: "findSupportedLock" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
