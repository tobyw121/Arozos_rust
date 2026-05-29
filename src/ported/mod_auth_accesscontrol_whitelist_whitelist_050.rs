//! Original Go file: `mod/auth/accesscontrol/whitelist/whitelist.go`
//! Package: `whitelist`; LOC: 119; SHA256: `2179c05b733e317550fecdce84def41ef805878d9eee302356f2e8154259b3d1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/whitelist/whitelist.go", package: "whitelist", go_loc: 119, functions: 6, types: 1, sha256: "2179c05b733e317550fecdce84def41ef805878d9eee302356f2e8154259b3d1" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/auth/accesscontrol",
    "imuslab.com/arozos/mod/database",
    "log",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("WhiteList", "struct", 17),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewWhitelistManager", "", 22),
    ("SetWhitelistEnabled", "wl *WhiteList", 39),
    ("IsWhitelisted", "wl *WhiteList", 49),
    ("ListWhitelistedIpRanges", "wl *WhiteList", 75),
    ("SetWhitelist", "wl *WhiteList", 92),
    ("UnsetWhitelist", "wl *WhiteList", 105),
];

pub async fn newwhitelistmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist.go", function: "NewWhitelistManager" })
}

pub async fn whitelist_setwhitelistenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist.go", function: "WhiteList.SetWhitelistEnabled" })
}

pub async fn whitelist_iswhitelisted(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist.go", function: "WhiteList.IsWhitelisted" })
}

pub async fn whitelist_listwhitelistedipranges(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist.go", function: "WhiteList.ListWhitelistedIpRanges" })
}

pub async fn whitelist_setwhitelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist.go", function: "WhiteList.SetWhitelist" })
}

pub async fn whitelist_unsetwhitelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist.go", function: "WhiteList.UnsetWhitelist" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
