//! Original Go file: `mod/compatibility/browser.go`
//! Package: `compatibility`; LOC: 60; SHA256: `f2689052ca50086fa376a83f2c308b23772c03d267566770cf5189f5b85464b3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/compatibility/browser.go", package: "compatibility", go_loc: 60, functions: 2, types: 0, sha256: "f2689052ca50086fa376a83f2c308b23772c03d267566770cf5189f5b85464b3" };

pub const GO_IMPORTS: &[&str] = &[
    "path/filepath",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FirefoxBrowserVersionForBypassUploadMetaHeaderCheck", "", 18),
    ("BrowserCompatibilityOverrideContentType", "", 46),
];

pub async fn firefoxbrowserversionforbypassuploadmetaheadercheck(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/compatibility/browser.go", function: "FirefoxBrowserVersionForBypassUploadMetaHeaderCheck" })
}

pub async fn browsercompatibilityoverridecontenttype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/compatibility/browser.go", function: "BrowserCompatibilityOverrideContentType" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
