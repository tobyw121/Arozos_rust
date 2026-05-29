//! Original Go file: `mod/network/webdav/xml.go`
//! Package: `webdav`; LOC: 519; SHA256: `0f0ff6ac115cac7beb6a9185ac5b43c15780b40747b57c97b9223871a0d00fb0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/xml.go", package: "webdav", go_loc: 519, functions: 15, types: 16, sha256: "0f0ff6ac115cac7beb6a9185ac5b43c15780b40747b57c97b9223871a0d00fb0" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "encoding/xml",
    "fmt",
    "imuslab.com/arozos/mod/network/webdav/internal/xml",
    "io",
    "net/http",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("lockInfo", "struct", 39),
    ("owner", "struct", 48),
    ("countingReader", "struct", 73),
    ("propfindProps", "[", 137),
    ("propfind", "struct", 170),
    ("Property", "struct", 209),
    ("ixmlProperty", "struct", 229),
    ("xmlError", "struct", 237),
    ("propstat", "struct", 244),
    ("ixmlPropstat", "struct", 253),
    ("response", "struct", 291),
    ("multistatusWriter", "struct", 309),
    ("xmlValue", "[", 414),
    ("proppatchProps", "[", 444),
    ("setRemove", "struct", 483),
    ("propertyupdate", "struct", 490),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("readLockInfo", "", 52),
    ("Read", "c *countingReader", 78),
    ("writeLockInfo", "", 84),
    ("escape", "", 104),
    ("next", "", 121),
    ("UnmarshalXML", "pn *propfindProps", 143),
    ("readPropfind", "", 178),
    ("MarshalXML", "ps propstat", 262),
    ("write", "w *multistatusWriter", 328),
    ("writeHeader", "w *multistatusWriter", 351),
    ("close", "w *multistatusWriter", 378),
    ("xmlLang", "", 405),
    ("UnmarshalXML", "v *xmlValue", 416),
    ("UnmarshalXML", "ps *proppatchProps", 454),
    ("readProppatch", "", 496),
];

pub async fn readlockinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "readLockInfo" })
}

pub async fn countingreader_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "countingReader.Read" })
}

pub async fn writelockinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "writeLockInfo" })
}

pub async fn escape(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "escape" })
}

pub async fn next(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "next" })
}

pub async fn propfindprops_unmarshalxml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "propfindProps.UnmarshalXML" })
}

pub async fn readpropfind(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "readPropfind" })
}

pub async fn propstat_marshalxml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "propstat.MarshalXML" })
}

pub async fn multistatuswriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "multistatusWriter.write" })
}

pub async fn multistatuswriter_writeheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "multistatusWriter.writeHeader" })
}

pub async fn multistatuswriter_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "multistatusWriter.close" })
}

pub async fn xmllang(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "xmlLang" })
}

pub async fn xmlvalue_unmarshalxml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "xmlValue.UnmarshalXML" })
}

pub async fn proppatchprops_unmarshalxml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "proppatchProps.UnmarshalXML" })
}

pub async fn readproppatch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/xml.go", function: "readProppatch" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
