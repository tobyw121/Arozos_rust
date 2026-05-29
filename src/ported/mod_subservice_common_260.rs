//! Original Go file: `mod/subservice/common.go`
//! Package: `subservice`; LOC: 196; SHA256: `fdea077eba187666384324306b9a5073d004546ff97c64f1db5979bc4bff0b94`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/subservice/common.go", package: "subservice", go_loc: 196, functions: 18, types: 0, sha256: "fdea077eba187666384324306b9a5073d004546ff97c64f1db5979bc4bff0b94" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "encoding/base64",
    "errors",
    "io",
    "log",
    "net/http",
    "os",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("sendTextResponse", "", 31),
    ("sendJSONResponse", "", 36),
    ("sendErrorResponse", "", 41),
    ("sendOK", "", 46),
    ("mv", "", 62),
    ("stringInSlice", "", 88),
    ("fileExists", "", 97),
    ("isDir", "", 105),
    ("inArray", "", 123),
    ("timeToString", "", 132),
    ("intToString", "", 136),
    ("stringToInt", "", 140),
    ("stringToInt64", "", 144),
    ("int64ToString", "", 152),
    ("getUnixTime", "", 157),
    ("loadImageAsBase64", "", 161),
    ("pushToSliceIfNotExist", "", 172),
    ("removeFromSliceIfExists", "", 187),
];

pub async fn sendtextresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "sendTextResponse" })
}

pub async fn sendjsonresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "sendJSONResponse" })
}

pub async fn senderrorresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "sendErrorResponse" })
}

pub async fn sendok(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "sendOK" })
}

pub async fn mv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "mv" })
}

pub async fn stringinslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "stringInSlice" })
}

pub async fn fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "fileExists" })
}

pub async fn isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "isDir" })
}

pub async fn inarray(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "inArray" })
}

pub async fn timetostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "timeToString" })
}

pub async fn inttostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "intToString" })
}

pub async fn stringtoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "stringToInt" })
}

pub async fn stringtoint64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "stringToInt64" })
}

pub async fn int64tostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "int64ToString" })
}

pub async fn getunixtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "getUnixTime" })
}

pub async fn loadimageasbase64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "loadImageAsBase64" })
}

pub async fn pushtosliceifnotexist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "pushToSliceIfNotExist" })
}

pub async fn removefromsliceifexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/common.go", function: "removeFromSliceIfExists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
