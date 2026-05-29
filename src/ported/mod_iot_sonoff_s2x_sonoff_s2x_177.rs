//! Original Go file: `mod/iot/sonoff_s2x/sonoff_s2x.go`
//! Package: `sonoff_s2x`; LOC: 178; SHA256: `dc196b3ac2285677118f3b8cd7045b68f35780202c0ba18da6190b0b8c948add`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/sonoff_s2x/sonoff_s2x.go", package: "sonoff_s2x", go_loc: 178, functions: 9, types: 1, sha256: "dc196b3ac2285677118f3b8cd7045b68f35780202c0ba18da6190b0b8c948add" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/iot",
    "imuslab.com/arozos/mod/network/mdns",
    "log",
    "regexp",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Handler", "struct", 23),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewProtocolHandler", "", 29),
    ("Start", "h *Handler", 37),
    ("Scan", "h *Handler", 42),
    ("Connect", "h *Handler", 127),
    ("Disconnect", "h *Handler", 131),
    ("Status", "h *Handler", 135),
    ("Icon", "h *Handler", 153),
    ("Execute", "h *Handler", 157),
    ("Stats", "h *Handler", 167),
];

pub async fn newprotocolhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "NewProtocolHandler" })
}

pub async fn handler_start(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Start" })
}

pub async fn handler_scan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Scan" })
}

pub async fn handler_connect(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Connect" })
}

pub async fn handler_disconnect(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Disconnect" })
}

pub async fn handler_status(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Status" })
}

pub async fn handler_icon(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Icon" })
}

pub async fn handler_execute(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Execute" })
}

pub async fn handler_stats(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/sonoff_s2x/sonoff_s2x.go", function: "Handler.Stats" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
