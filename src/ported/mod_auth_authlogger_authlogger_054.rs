//! Original Go file: `mod/auth/authlogger/authlogger.go`
//! Package: `authlogger`; LOC: 171; SHA256: `bf6d0ff5e0406fe59196b373529b5edc190e4e4bc380e0b0b036e6ae66505ab0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/authlogger/authlogger.go", package: "authlogger", go_loc: 171, functions: 7, types: 2, sha256: "bf6d0ff5e0406fe59196b373529b5edc190e4e4bc380e0b0b036e6ae66505ab0" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Logger", "struct", 26),
    ("LoginRecord", "struct", 30),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewLogger", "", 40),
    ("LogAuth", "l *Logger", 52),
    ("LogAuthByRequestInfo", "l *Logger", 69),
    ("Close", "l *Logger", 126),
    ("ListSummary", "l *Logger", 131),
    ("ListRecords", "l *Logger", 141),
    ("getIpAddressFromRequest", "", 164),
];

pub async fn newlogger(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "NewLogger" })
}

pub async fn logger_logauth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "Logger.LogAuth" })
}

pub async fn logger_logauthbyrequestinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "Logger.LogAuthByRequestInfo" })
}

pub async fn logger_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "Logger.Close" })
}

pub async fn logger_listsummary(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "Logger.ListSummary" })
}

pub async fn logger_listrecords(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "Logger.ListRecords" })
}

pub async fn getipaddressfromrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger.go", function: "getIpAddressFromRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
