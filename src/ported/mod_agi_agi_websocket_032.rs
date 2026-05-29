//! Original Go file: `mod/agi/agi.websocket.go`
//! Package: `agi`; LOC: 234; SHA256: `4863ba637e3713c89770a85d9a0ca98ee084ad8c2b3a7b1c847b600497f387ed`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.websocket.go", package: "agi", go_loc: 234, functions: 2, types: 0, sha256: "4863ba637e3713c89770a85d9a0ca98ee084ad8c2b3a7b1c847b600497f387ed" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/gorilla/websocket",
    "github.com/robertkrimen/otto",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/user",
    "log",
    "net/http",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("checkWebSocketConnectionUpgradeStatus", "", 35),
    ("injectWebSocketFunctions", "g *Gateway", 63),
];

pub async fn checkwebsocketconnectionupgradestatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.websocket.go", function: "checkWebSocketConnectionUpgradeStatus" })
}

pub async fn gateway_injectwebsocketfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.websocket.go", function: "Gateway.injectWebSocketFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
