//! Original Go file: `mod/network/webdav/internal/xml/read.go`
//! Package: `xml`; LOC: 692; SHA256: `68dbe783fe58a7bb6a89ac9fde9bd0d1b9c281990ff292b83913fe90aec3ce8b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/read.go", package: "xml", go_loc: 692, functions: 12, types: 3, sha256: "68dbe783fe58a7bb6a89ac9fde9bd0d1b9c281990ff292b83913fe90aec3ce8b" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "encoding",
    "errors",
    "fmt",
    "reflect",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("UnmarshalError", "s", 137),
    ("Unmarshaler", "interface", 156),
    ("UnmarshalerAttr", "interface", 168),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Unmarshal", "", 114),
    ("Decode", "d *Decoder", 120),
    ("DecodeElement", "d *Decoder", 128),
    ("Error", "e UnmarshalError", 139),
    ("receiverType", "", 173),
    ("unmarshalInterface", "p *Decoder", 183),
    ("unmarshalTextInterface", "p *Decoder", 205),
    ("unmarshalAttr", "p *Decoder", 228),
    ("unmarshal", "p *Decoder", 272),
    ("copyValue", "", 555),
    ("unmarshalPath", "p *Decoder", 612),
    ("Skip", "d *Decoder", 677),
];

pub async fn unmarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Unmarshal" })
}

pub async fn decoder_decode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.Decode" })
}

pub async fn decoder_decodeelement(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.DecodeElement" })
}

pub async fn unmarshalerror_error(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "UnmarshalError.Error" })
}

pub async fn receivertype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "receiverType" })
}

pub async fn decoder_unmarshalinterface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.unmarshalInterface" })
}

pub async fn decoder_unmarshaltextinterface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.unmarshalTextInterface" })
}

pub async fn decoder_unmarshalattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.unmarshalAttr" })
}

pub async fn decoder_unmarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.unmarshal" })
}

pub async fn copyvalue(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "copyValue" })
}

pub async fn decoder_unmarshalpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.unmarshalPath" })
}

pub async fn decoder_skip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read.go", function: "Decoder.Skip" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
