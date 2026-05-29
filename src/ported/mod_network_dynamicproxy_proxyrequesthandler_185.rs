//! Original Go file: `mod/network/dynamicproxy/proxyRequestHandler.go`
//! Package: `dynamicproxy`; LOC: 99; SHA256: `baaf4d503c0eea475953d4c03050b866c9eb6674ae0ae82aa67683924f00881d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/dynamicproxy/proxyRequestHandler.go", package: "dynamicproxy", go_loc: 99, functions: 5, types: 0, sha256: "baaf4d503c0eea475953d4c03050b866c9eb6674ae0ae82aa67683924f00881d" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/network/websocketproxy",
    "log",
    "net/http",
    "net/url",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getTargetProxyEndpointFromRequestURI", "router *Router", 11),
    ("getSubdomainProxyEndpointFromHostname", "router *Router", 25),
    ("rewriteURL", "router *Router", 35),
    ("subdomainRequest", "h *ProxyHandler", 42),
    ("proxyRequest", "h *ProxyHandler", 74),
];

pub async fn router_gettargetproxyendpointfromrequesturi(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/proxyRequestHandler.go", function: "Router.getTargetProxyEndpointFromRequestURI" })
}

pub async fn router_getsubdomainproxyendpointfromhostname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/proxyRequestHandler.go", function: "Router.getSubdomainProxyEndpointFromHostname" })
}

pub async fn router_rewriteurl(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/proxyRequestHandler.go", function: "Router.rewriteURL" })
}

pub async fn proxyhandler_subdomainrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/proxyRequestHandler.go", function: "ProxyHandler.subdomainRequest" })
}

pub async fn proxyhandler_proxyrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/proxyRequestHandler.go", function: "ProxyHandler.proxyRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
