//! Original Go file: `mod/network/websocketproxy/websocketproxy.go`
//! Package: `websocketproxy`; LOC: 234; SHA256: `3c2b22c1953b407632d923a9fb379eb0f43b61ecc4305154fcd76c3e05b34aed`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/websocketproxy/websocketproxy.go", package: "websocketproxy", go_loc: 234, functions: 5, types: 1, sha256: "3c2b22c1953b407632d923a9fb379eb0f43b61ecc4305154fcd76c3e05b34aed" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "github.com/gorilla/websocket",
    "io",
    "log",
    "net",
    "net/http",
    "net/url",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("WebsocketProxy", "struct", 30),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ProxyHandler", "", 52),
    ("NewProxy", "", 56),
    ("ServeHTTP", "w *WebsocketProxy", 69),
    ("copyHeader", "", 219),
    ("copyResponse", "", 227),
];

pub async fn proxyhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocketproxy/websocketproxy.go", function: "ProxyHandler" })
}

pub async fn newproxy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocketproxy/websocketproxy.go", function: "NewProxy" })
}

pub async fn websocketproxy_servehttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocketproxy/websocketproxy.go", function: "WebsocketProxy.ServeHTTP" })
}

pub async fn copyheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocketproxy/websocketproxy.go", function: "copyHeader" })
}

pub async fn copyresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocketproxy/websocketproxy.go", function: "copyResponse" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
