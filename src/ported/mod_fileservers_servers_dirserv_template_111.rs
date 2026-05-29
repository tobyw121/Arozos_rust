//! Original Go file: `mod/fileservers/servers/dirserv/template.go`
//! Package: `dirserv`; LOC: 139; SHA256: `98a8a2fc527fb860c506135d19fc78fa4fa14defa0c527fca5c3721ecbe51158`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/dirserv/template.go", package: "dirserv", go_loc: 139, functions: 5, types: 0, sha256: "98a8a2fc527fb860c506135d19fc78fa4fa14defa0c527fca5c3721ecbe51158" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/hidden",
    "mime",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getPageHeader", "", 13),
    ("getItemHTML", "", 63),
    ("getBackButton", "", 108),
    ("getPageFooter", "", 116),
    ("byteCountIEC", "", 127),
];

pub async fn getpageheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/template.go", function: "getPageHeader" })
}

pub async fn getitemhtml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/template.go", function: "getItemHTML" })
}

pub async fn getbackbutton(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/template.go", function: "getBackButton" })
}

pub async fn getpagefooter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/template.go", function: "getPageFooter" })
}

pub async fn bytecountiec(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/template.go", function: "byteCountIEC" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
