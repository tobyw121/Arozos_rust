//! Original Go file: `mod/agi/static/static.go`
//! Package: `static`; LOC: 131; SHA256: `35c91855adb201518b6e5f23de9ccc040713f5c70b6ce4c16b887711b64743c1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/static/static.go", package: "static", go_loc: 131, functions: 8, types: 1, sha256: "35c91855adb201518b6e5f23de9ccc040713f5c70b6ce4c16b887711b64743c1" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "net/url",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("AgiLibInjectionPayload", "struct", 18),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("RelativeVpathRewrite", "", 29),
    ("VirtualPathToRealPath", "", 63),
    ("RealpathToVirtualpath", "", 75),
    ("CheckUserAccessToScript", "", 80),
    ("IsValidAGIScript", "", 89),
    ("GetScriptRoot", "", 94),
    ("SpecialURIDecode", "", 106),
    ("CheckRootEscape", "", 114),
];

pub async fn relativevpathrewrite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "RelativeVpathRewrite" })
}

pub async fn virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "VirtualPathToRealPath" })
}

pub async fn realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "RealpathToVirtualpath" })
}

pub async fn checkuseraccesstoscript(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "CheckUserAccessToScript" })
}

pub async fn isvalidagiscript(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "IsValidAGIScript" })
}

pub async fn getscriptroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "GetScriptRoot" })
}

pub async fn specialuridecode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "SpecialURIDecode" })
}

pub async fn checkrootescape(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/static.go", function: "CheckRootEscape" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
