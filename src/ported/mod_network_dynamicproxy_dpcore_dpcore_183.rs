//! Original Go file: `mod/network/dynamicproxy/dpcore/dpcore.go`
//! Package: `dpcore`; LOC: 394; SHA256: `8199d87e80f27d8f30a6ea14af0982012f8667ab5e188550d921942e3cb0592a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/dynamicproxy/dpcore/dpcore.go", package: "dpcore", go_loc: 394, functions: 13, types: 4, sha256: "8199d87e80f27d8f30a6ea14af0982012f8667ab5e188550d921942e3cb0592a" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "io",
    "log",
    "net",
    "net/http",
    "net/url",
    "path/filepath",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ReverseProxy", "struct", 26),
    ("requestCanceler", "interface", 63),
    ("writeFlusher", "interface", 147),
    ("maxLatencyWriter", "struct", 152),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDynamicProxyCore", "", 67),
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
    ("ProxyHTTPS", "p *ReverseProxy", 325),
    ("ServeHTTP", "p *ReverseProxy", 386),
];

pub async fn newdynamicproxycore(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "NewDynamicProxyCore" })
}

pub async fn singlejoiningslash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "singleJoiningSlash" })
}

pub async fn copyheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "copyHeader" })
}

pub async fn reverseproxy_copyresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "ReverseProxy.copyResponse" })
}

pub async fn maxlatencywriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "maxLatencyWriter.Write" })
}

pub async fn maxlatencywriter_flushloop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "maxLatencyWriter.flushLoop" })
}

pub async fn maxlatencywriter_stop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "maxLatencyWriter.stop" })
}

pub async fn reverseproxy_logf(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "ReverseProxy.logf" })
}

pub async fn removeheaders(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "removeHeaders" })
}

pub async fn addxforwardedforheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "addXForwardedForHeader" })
}

pub async fn reverseproxy_proxyhttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "ReverseProxy.ProxyHTTP" })
}

pub async fn reverseproxy_proxyhttps(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "ReverseProxy.ProxyHTTPS" })
}

pub async fn reverseproxy_servehttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dpcore/dpcore.go", function: "ReverseProxy.ServeHTTP" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
