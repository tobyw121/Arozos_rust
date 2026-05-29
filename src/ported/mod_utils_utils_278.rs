//! Original Go file: `mod/utils/utils.go`
//! Package: `utils`; LOC: 238; SHA256: `2fa68fe82cb1808959ff1e0afc902f59ff84ea2f22741620a0f1f6a8c7af04c5`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/utils/utils.go", package: "utils", go_loc: 238, functions: 20, types: 0, sha256: "2fa68fe82cb1808959ff1e0afc902f59ff84ea2f22741620a0f1f6a8c7af04c5" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "encoding/base64",
    "errors",
    "io",
    "log",
    "net/http",
    "os",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SendTextResponse", "", 24),
    ("SendJSONResponse", "", 29),
    ("SendErrorResponse", "", 34),
    ("SendOK", "", 39),
    ("GetPara", "", 45),
    ("GetBool", "", 54),
    ("GetInt", "", 72),
    ("PostPara", "", 88),
    ("PostBool", "", 98),
    ("PostInt", "", 116),
    ("FileExists", "", 131),
    ("IsDir", "", 139),
    ("TimeToString", "", 157),
    ("LoadImageAsBase64", "", 161),
    ("ConstructRelativePathFromRequestURL", "", 173),
    ("StringInArray", "", 186),
    ("StringInArrayIgnoreCase", "", 195),
    ("Templateload", "", 205),
    ("TemplateApply", "", 220),
    ("FilenameIsWebSafe", "", 230),
];

pub async fn sendtextresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "SendTextResponse" })
}

pub async fn sendjsonresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "SendJSONResponse" })
}

pub async fn senderrorresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "SendErrorResponse" })
}

pub async fn sendok(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "SendOK" })
}

pub async fn getpara(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "GetPara" })
}

pub async fn getbool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "GetBool" })
}

pub async fn getint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "GetInt" })
}

pub async fn postpara(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "PostPara" })
}

pub async fn postbool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "PostBool" })
}

pub async fn postint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "PostInt" })
}

pub async fn fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "FileExists" })
}

pub async fn isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "IsDir" })
}

pub async fn timetostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "TimeToString" })
}

pub async fn loadimageasbase64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "LoadImageAsBase64" })
}

pub async fn constructrelativepathfromrequesturl(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "ConstructRelativePathFromRequestURL" })
}

pub async fn stringinarray(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "StringInArray" })
}

pub async fn stringinarrayignorecase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "StringInArrayIgnoreCase" })
}

pub async fn templateload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "Templateload" })
}

pub async fn templateapply(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "TemplateApply" })
}

pub async fn filenameiswebsafe(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils.go", function: "FilenameIsWebSafe" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
