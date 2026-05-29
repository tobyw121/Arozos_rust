//! Original Go file: `mod/storage/webdav/common.go`
//! Package: `webdav`; LOC: 170; SHA256: `1a0e84de9cd59696f4464f2a36a32b1089b0dc794e232664b8a5a8b951fc65de`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/webdav/common.go", package: "webdav", go_loc: 170, functions: 13, types: 0, sha256: "1a0e84de9cd59696f4464f2a36a32b1089b0dc794e232664b8a5a8b951fc65de" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "encoding/base64",
    "errors",
    "io",
    "log",
    "net/http",
    "os",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("sendTextResponse", "", 30),
    ("sendJSONResponse", "", 35),
    ("sendErrorResponse", "", 40),
    ("sendOK", "", 45),
    ("mv", "", 61),
    ("stringInSlice", "", 87),
    ("fileExists", "", 96),
    ("isDir", "", 104),
    ("inArray", "", 122),
    ("timeToString", "", 131),
    ("loadImageAsBase64", "", 135),
    ("pushToSliceIfNotExist", "", 146),
    ("removeFromSliceIfExists", "", 161),
];

pub async fn sendtextresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "sendTextResponse" })
}

pub async fn sendjsonresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "sendJSONResponse" })
}

pub async fn senderrorresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "sendErrorResponse" })
}

pub async fn sendok(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "sendOK" })
}

pub async fn mv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "mv" })
}

pub async fn stringinslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "stringInSlice" })
}

pub async fn fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "fileExists" })
}

pub async fn isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "isDir" })
}

pub async fn inarray(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "inArray" })
}

pub async fn timetostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "timeToString" })
}

pub async fn loadimageasbase64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "loadImageAsBase64" })
}

pub async fn pushtosliceifnotexist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "pushToSliceIfNotExist" })
}

pub async fn removefromsliceifexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/webdav/common.go", function: "removeFromSliceIfExists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
