//! Original Go file: `mod/network/reverseproxy/reverse.go`
//! Package: `reverseproxy`; LOC: 387; SHA256: `7772491ce9c583bf932fcd6758ecf988caac0b67c6922d656ffd37055c8f8cf2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/reverseproxy/reverse.go", package: "reverseproxy", go_loc: 387, functions: 13, types: 4, sha256: "7772491ce9c583bf932fcd6758ecf988caac0b67c6922d656ffd37055c8f8cf2" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "io",
    "log",
    "net",
    "net/http",
    "net/url",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ReverseProxy", "struct", 24),
    ("requestCanceler", "interface", 57),
    ("writeFlusher", "interface", 147),
    ("maxLatencyWriter", "struct", 152),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewReverseProxy", "", 70),
    ("singleJoiningSlash", "", 95),
    ("copyHeader", "", 107),
    ("copyResponse", "p *ReverseProxy", 129),
    ("Write", "m *maxLatencyWriter", 159),
    ("flushLoop", "m *maxLatencyWriter", 165),
    ("stop", "m *maxLatencyWriter", 183),
    ("logf", "p *ReverseProxy", 187),
    ("removeHeaders", "", 195),
    ("addXForwardedForHeader", "", 218),
    ("ProxyHTTP", "p *ReverseProxy", 230),
    ("ProxyHTTPS", "p *ReverseProxy", 318),
    ("ServeHTTP", "p *ReverseProxy", 379),
];

pub async fn newreverseproxy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "NewReverseProxy" })
}

pub async fn singlejoiningslash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "singleJoiningSlash" })
}

pub async fn copyheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "copyHeader" })
}

pub async fn reverseproxy_copyresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "ReverseProxy.copyResponse" })
}

pub async fn maxlatencywriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "maxLatencyWriter.Write" })
}

pub async fn maxlatencywriter_flushloop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "maxLatencyWriter.flushLoop" })
}

pub async fn maxlatencywriter_stop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "maxLatencyWriter.stop" })
}

pub async fn reverseproxy_logf(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "ReverseProxy.logf" })
}

pub async fn removeheaders(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "removeHeaders" })
}

pub async fn addxforwardedforheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "addXForwardedForHeader" })
}

pub async fn reverseproxy_proxyhttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "ReverseProxy.ProxyHTTP" })
}

pub async fn reverseproxy_proxyhttps(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "ReverseProxy.ProxyHTTPS" })
}

pub async fn reverseproxy_servehttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/reverseproxy/reverse.go", function: "ReverseProxy.ServeHTTP" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
