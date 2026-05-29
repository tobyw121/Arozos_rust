//! Original Go file: `mod/network/websocket/websocket.go`
//! Package: `websocket`; LOC: 16; SHA256: `e7d83f7c2eb6f29eede82db995f41e655cd6e9970a2a1e9148137a67003fd940`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/websocket/websocket.go", package: "websocket", go_loc: 16, functions: 2, types: 1, sha256: "e7d83f7c2eb6f29eede82db995f41e655cd6e9970a2a1e9148137a67003fd940" };

pub const GO_IMPORTS: &[&str] = &[
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Router", "struct", 5),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewRouter", "", 8),
    ("HandleWebSocketRouting", "s *Router", 13),
];

pub async fn newrouter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocket/websocket.go", function: "NewRouter" })
}

pub async fn router_handlewebsocketrouting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocket/websocket.go", function: "Router.HandleWebSocketRouting" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
