//! Original Go file: `mod/network/webdav/webdav_test.go`
//! Package: `webdav`; LOC: 348; SHA256: `0a144d789c9b11f49d8be99809289722d65b243e50c2c960cd089d8bdcc91620`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/webdav_test.go", package: "webdav", go_loc: 348, functions: 3, types: 0, sha256: "0a144d789c9b11f49d8be99809289722d65b243e50c2c960cd089d8bdcc91620" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "errors",
    "fmt",
    "io",
    "net/http",
    "net/http/httptest",
    "net/url",
    "os",
    "reflect",
    "regexp",
    "sort",
    "strings",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestPrefix", "", 24),
    ("TestEscapeXML", "", 206),
    ("TestFilenameEscape", "", 244),
];

pub async fn testprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav_test.go", function: "TestPrefix" })
}

pub async fn testescapexml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav_test.go", function: "TestEscapeXML" })
}

pub async fn testfilenameescape(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/webdav_test.go", function: "TestFilenameEscape" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
