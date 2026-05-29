//! Original Go file: `mod/updates/handler.go`
//! Package: `updates`; LOC: 182; SHA256: `8fad7f9f94852bee2e00ab264d5ef1d176eb6b46ef42d2845fd5924207d2e365`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/updates/handler.go", package: "updates", go_loc: 182, functions: 4, types: 1, sha256: "8fad7f9f94852bee2e00ab264d5ef1d176eb6b46ef42d2845fd5924207d2e365" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "fmt",
    "github.com/gorilla/websocket",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "runtime",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("UpdateConfig", "struct", 16),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleUpdateCheckSize", "", 47),
    ("HandleUpdateDownloadRequest", "", 70),
    ("HandleGetUpdatePlatformInfo", "", 141),
    ("HandlePendingCheck", "", 175),
];

pub async fn handleupdatechecksize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/handler.go", function: "HandleUpdateCheckSize" })
}

pub async fn handleupdatedownloadrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/handler.go", function: "HandleUpdateDownloadRequest" })
}

pub async fn handlegetupdateplatforminfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/handler.go", function: "HandleGetUpdatePlatformInfo" })
}

pub async fn handlependingcheck(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/handler.go", function: "HandlePendingCheck" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
