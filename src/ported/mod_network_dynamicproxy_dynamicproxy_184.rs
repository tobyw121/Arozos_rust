//! Original Go file: `mod/network/dynamicproxy/dynamicproxy.go`
//! Package: `dynamicproxy`; LOC: 195; SHA256: `d5238fa1dad9614922595025ef497e3b41f43f88d5a876b99d3f282fa528eded`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/dynamicproxy/dynamicproxy.go", package: "dynamicproxy", go_loc: 195, functions: 6, types: 5, sha256: "d5238fa1dad9614922595025ef497e3b41f43f88d5a876b99d3f282fa528eded" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "errors",
    "imuslab.com/arozos/mod/network/dynamicproxy/dpcore",
    "imuslab.com/arozos/mod/network/reverseproxy",
    "log",
    "net/http",
    "net/url",
    "strconv",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Router", "struct", 22),
    ("RouterOption", "struct", 33),
    ("ProxyEndpoint", "struct", 37),
    ("SubdomainEndpoint", "struct", 44),
    ("ProxyHandler", "struct", 51),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDynamicProxy", "", 55),
    ("StartProxyService", "router *Router", 75),
    ("StopProxyService", "router *Router", 95),
    ("AddProxyService", "router *Router", 115),
    ("SetRootProxy", "router *Router", 148),
    ("ServeHTTP", "h *ProxyHandler", 179),
];

pub async fn newdynamicproxy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dynamicproxy.go", function: "NewDynamicProxy" })
}

pub async fn router_startproxyservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dynamicproxy.go", function: "Router.StartProxyService" })
}

pub async fn router_stopproxyservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dynamicproxy.go", function: "Router.StopProxyService" })
}

pub async fn router_addproxyservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dynamicproxy.go", function: "Router.AddProxyService" })
}

pub async fn router_setrootproxy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dynamicproxy.go", function: "Router.SetRootProxy" })
}

pub async fn proxyhandler_servehttp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/dynamicproxy.go", function: "ProxyHandler.ServeHTTP" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
