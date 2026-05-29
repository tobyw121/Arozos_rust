//! Original Go file: `mod/auth/accesscontrol/blacklist/handler.go`
//! Package: `blacklist`; LOC: 96; SHA256: `2a7bec330f760ba9ee9655cdd9cab9c8a61f2f01035f972b47cbc5f6d89d7b61`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/blacklist/handler.go", package: "blacklist", go_loc: 96, functions: 6, types: 0, sha256: "2a7bec330f760ba9ee9655cdd9cab9c8a61f2f01035f972b47cbc5f6d89d7b61" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/network",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleAddBannedIP", "bl *BlackList", 17),
    ("HandleRemoveBannedIP", "bl *BlackList", 33),
    ("HandleSetBlacklistEnable", "bl *BlackList", 49),
    ("SetBlacklistEnabled", "bl *BlackList", 68),
    ("HandleListBannedIPs", "bl *BlackList", 78),
    ("CheckIsBannedByRequest", "bl *BlackList", 84),
];

pub async fn blacklist_handleaddbannedip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/handler.go", function: "BlackList.HandleAddBannedIP" })
}

pub async fn blacklist_handleremovebannedip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/handler.go", function: "BlackList.HandleRemoveBannedIP" })
}

pub async fn blacklist_handlesetblacklistenable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/handler.go", function: "BlackList.HandleSetBlacklistEnable" })
}

pub async fn blacklist_setblacklistenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/handler.go", function: "BlackList.SetBlacklistEnabled" })
}

pub async fn blacklist_handlelistbannedips(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/handler.go", function: "BlackList.HandleListBannedIPs" })
}

pub async fn blacklist_checkisbannedbyrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/handler.go", function: "BlackList.CheckIsBannedByRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
