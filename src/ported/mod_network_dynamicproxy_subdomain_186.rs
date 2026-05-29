//! Original Go file: `mod/network/dynamicproxy/subdomain.go`
//! Package: `dynamicproxy`; LOC: 44; SHA256: `2b13f50ecfc1dcf2ef183e97ba08a1fbf382476b95fbbd572b6c4c419e9431b0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/dynamicproxy/subdomain.go", package: "dynamicproxy", go_loc: 44, functions: 1, types: 0, sha256: "2b13f50ecfc1dcf2ef183e97ba08a1fbf382476b95fbbd572b6c4c419e9431b0" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/network/reverseproxy",
    "log",
    "net/url",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("AddSubdomainRoutingService", "router *Router", 15),
];

pub async fn router_addsubdomainroutingservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/dynamicproxy/subdomain.go", function: "Router.AddSubdomainRoutingService" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
