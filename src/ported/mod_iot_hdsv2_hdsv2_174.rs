//! Original Go file: `mod/iot/hdsv2/hdsv2.go`
//! Package: `hdsv2`; LOC: 225; SHA256: `635a9bc5fc3999ff36cfb9a99b9449822ca36de5c708380b53c4e79a1f3d9172`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/hdsv2/hdsv2.go", package: "hdsv2", go_loc: 225, functions: 11, types: 1, sha256: "635a9bc5fc3999ff36cfb9a99b9449822ca36de5c708380b53c4e79a1f3d9172" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/iot",
    "imuslab.com/arozos/mod/network/mdns",
    "io",
    "log",
    "net/http",
    "net/url",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Handler", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewProtocolHandler", "", 31),
    ("Start", "h *Handler", 39),
    ("Scan", "h *Handler", 45),
    ("Connect", "h *Handler", 99),
    ("Disconnect", "h *Handler", 104),
    ("Status", "h *Handler", 109),
    ("Icon", "h *Handler", 114),
    ("Execute", "h *Handler", 128),
    ("Stats", "h *Handler", 150),
    ("getEndpoints", "", 164),
    ("getStatusForDevice", "", 197),
];

pub async fn newprotocolhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "NewProtocolHandler" })
}

pub async fn handler_start(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Start" })
}

pub async fn handler_scan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Scan" })
}

pub async fn handler_connect(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Connect" })
}

pub async fn handler_disconnect(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Disconnect" })
}

pub async fn handler_status(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Status" })
}

pub async fn handler_icon(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Icon" })
}

pub async fn handler_execute(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Execute" })
}

pub async fn handler_stats(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "Handler.Stats" })
}

pub async fn getendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "getEndpoints" })
}

pub async fn getstatusfordevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hdsv2/hdsv2.go", function: "getStatusForDevice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
