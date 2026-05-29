//! Original Go file: `mod/network/webdav/lock.go`
//! Package: `webdav`; LOC: 445; SHA256: `a29231c7bb38de6f0d527ac84c6b2e3bfec449d0b6afef07d29f01601793f3c4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/lock.go", package: "webdav", go_loc: 445, functions: 20, types: 6, sha256: "a29231c7bb38de6f0d527ac84c6b2e3bfec449d0b6afef07d29f01601793f3c4" };

pub const GO_IMPORTS: &[&str] = &[
    "container/heap",
    "errors",
    "strconv",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Condition", "struct", 29),
    ("LockSystem", "interface", 38),
    ("LockDetails", "struct", 97),
    ("memLS", "struct", 123),
    ("memLSNode", "struct", 368),
    ("byExpiry", "[", 386),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewMemLS", "", 115),
    ("nextToken", "m *memLS", 133),
    ("collectExpiredNodes", "m *memLS", 138),
    ("Confirm", "m *memLS", 147),
    ("lookup", "m *memLS", 192),
    ("hold", "m *memLS", 212),
    ("unhold", "m *memLS", 222),
    ("Create", "m *memLS", 232),
    ("Refresh", "m *memLS", 252),
    ("Unlock", "m *memLS", 275),
    ("canCreate", "m *memLS", 291),
    ("create", "m *memLS", 315),
    ("remove", "m *memLS", 336),
    ("walkToRoot", "", 352),
    ("Len", "b *byExpiry", 388),
    ("Less", "b *byExpiry", 392),
    ("Swap", "b *byExpiry", 396),
    ("Push", "b *byExpiry", 402),
    ("Pop", "b *byExpiry", 408),
    ("parseTimeout", "", 421),
];

pub async fn newmemls(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "NewMemLS" })
}

pub async fn memls_nexttoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.nextToken" })
}

pub async fn memls_collectexpirednodes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.collectExpiredNodes" })
}

pub async fn memls_confirm(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.Confirm" })
}

pub async fn memls_lookup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.lookup" })
}

pub async fn memls_hold(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.hold" })
}

pub async fn memls_unhold(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.unhold" })
}

pub async fn memls_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.Create" })
}

pub async fn memls_refresh(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.Refresh" })
}

pub async fn memls_unlock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.Unlock" })
}

pub async fn memls_cancreate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.canCreate" })
}

pub async fn memls_create_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.create" })
}

pub async fn memls_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "memLS.remove" })
}

pub async fn walktoroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "walkToRoot" })
}

pub async fn byexpiry_len(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "byExpiry.Len" })
}

pub async fn byexpiry_less(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "byExpiry.Less" })
}

pub async fn byexpiry_swap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "byExpiry.Swap" })
}

pub async fn byexpiry_push(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "byExpiry.Push" })
}

pub async fn byexpiry_pop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "byExpiry.Pop" })
}

pub async fn parsetimeout(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock.go", function: "parseTimeout" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
