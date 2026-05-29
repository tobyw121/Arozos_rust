//! Original Go file: `mod/auth/explogin/explogin.go`
//! Package: `explogin`; LOC: 167; SHA256: `82b2d8724881ce49670eb1476f090a86de6b394b56351e5fd6901e2bee629a30`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/explogin/explogin.go", package: "explogin", go_loc: 167, functions: 7, types: 2, sha256: "82b2d8724881ce49670eb1476f090a86de6b394b56351e5fd6901e2bee629a30" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "math",
    "net",
    "net/http",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("UserLoginEntry", "struct", 21),
    ("ExpLoginHandler", "struct", 29),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewExponentialLoginHandler", "", 36),
    ("AllowImmediateAccess", "e *ExpLoginHandler", 47),
    ("AddUserRetrycount", "e *ExpLoginHandler", 74),
    ("ResetUserRetryCount", "e *ExpLoginHandler", 107),
    ("ResetAllUserRetryCounter", "e *ExpLoginHandler", 119),
    ("getDelayTimeFromRetryCount", "e *ExpLoginHandler", 127),
    ("getIpFromRequest", "", 142),
];

pub async fn newexponentialloginhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "NewExponentialLoginHandler" })
}

pub async fn exploginhandler_allowimmediateaccess(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "ExpLoginHandler.AllowImmediateAccess" })
}

pub async fn exploginhandler_adduserretrycount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "ExpLoginHandler.AddUserRetrycount" })
}

pub async fn exploginhandler_resetuserretrycount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "ExpLoginHandler.ResetUserRetryCount" })
}

pub async fn exploginhandler_resetalluserretrycounter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "ExpLoginHandler.ResetAllUserRetryCounter" })
}

pub async fn exploginhandler_getdelaytimefromretrycount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "ExpLoginHandler.getDelayTimeFromRetryCount" })
}

pub async fn getipfromrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin.go", function: "getIpFromRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
