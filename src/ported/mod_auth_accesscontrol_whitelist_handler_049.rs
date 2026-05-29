//! Original Go file: `mod/auth/accesscontrol/whitelist/handler.go`
//! Package: `whitelist`; LOC: 82; SHA256: `5367bbd4e0739e6cce8dccc011feff76985393f4385ad34bd31094f95228a086`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/whitelist/handler.go", package: "whitelist", go_loc: 82, functions: 5, types: 0, sha256: "5367bbd4e0739e6cce8dccc011feff76985393f4385ad34bd31094f95228a086" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/network",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleAddWhitelistedIP", "wl *WhiteList", 12),
    ("HandleRemoveWhitelistedIP", "wl *WhiteList", 28),
    ("HandleSetWhitelistEnable", "wl *WhiteList", 44),
    ("HandleListWhitelistedIPs", "wl *WhiteList", 64),
    ("CheckIsWhitelistedByRequest", "wl *WhiteList", 70),
];

pub async fn whitelist_handleaddwhitelistedip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/handler.go", function: "WhiteList.HandleAddWhitelistedIP" })
}

pub async fn whitelist_handleremovewhitelistedip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/handler.go", function: "WhiteList.HandleRemoveWhitelistedIP" })
}

pub async fn whitelist_handlesetwhitelistenable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/handler.go", function: "WhiteList.HandleSetWhitelistEnable" })
}

pub async fn whitelist_handlelistwhitelistedips(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/handler.go", function: "WhiteList.HandleListWhitelistedIPs" })
}

pub async fn whitelist_checkiswhitelistedbyrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/handler.go", function: "WhiteList.CheckIsWhitelistedByRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
