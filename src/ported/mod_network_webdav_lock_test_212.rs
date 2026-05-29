//! Original Go file: `mod/network/webdav/lock_test.go`
//! Package: `webdav`; LOC: 735; SHA256: `ab4638e9f600afb9a19687da9374bdf4d740ddddbb6de34ae03aca0e226e99e2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/lock_test.go", package: "webdav", go_loc: 735, functions: 10, types: 0, sha256: "ab4638e9f600afb9a19687da9374bdf4d740ddddbb6de34ae03aca0e226e99e2" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "math/rand",
    "path",
    "reflect",
    "sort",
    "strconv",
    "strings",
    "testing",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestWalkToRoot", "", 19),
    ("lockTestZeroDepth", "", 88),
    ("TestMemLSCanCreate", "", 98),
    ("TestMemLSLookup", "", 158),
    ("TestMemLSConfirm", "", 207),
    ("TestMemLSNonCanonicalRoot", "", 303),
    ("TestMemLSExpiry", "", 324),
    ("TestMemLS", "", 452),
    ("consistent", "m *memLS", 538),
    ("TestParseTimeout", "", 640),
];

pub async fn testwalktoroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestWalkToRoot" })
}

pub async fn locktestzerodepth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "lockTestZeroDepth" })
}

pub async fn testmemlscancreate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestMemLSCanCreate" })
}

pub async fn testmemlslookup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestMemLSLookup" })
}

pub async fn testmemlsconfirm(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestMemLSConfirm" })
}

pub async fn testmemlsnoncanonicalroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestMemLSNonCanonicalRoot" })
}

pub async fn testmemlsexpiry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestMemLSExpiry" })
}

pub async fn testmemls(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestMemLS" })
}

pub async fn memls_consistent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "memLS.consistent" })
}

pub async fn testparsetimeout(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/lock_test.go", function: "TestParseTimeout" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
