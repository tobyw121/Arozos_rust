//! Original Go file: `mod/info/logviewer/logviewer.go`
//! Package: `logviewer`; LOC: 123; SHA256: `ccca3a652229e2125362d9522a4d4179f255bf211eeff5430559bb0aaf35303a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/logviewer/logviewer.go", package: "logviewer", go_loc: 123, functions: 5, types: 3, sha256: "ccca3a652229e2125362d9522a4d4179f255bf211eeff5430559bb0aaf35303a" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/utils",
    "io/fs",
    "net/http",
    "os",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ViewerOption", "struct", 16),
    ("Viewer", "struct", 21),
    ("LogFile", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewLogViewer", "", 32),
    ("HandleListLog", "v *Viewer", 40),
    ("HandleReadLog", "v *Viewer", 48),
    ("ListLogFiles", "v *Viewer", 74),
    ("LoadLogFile", "v *Viewer", 110),
];

pub async fn newlogviewer(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/logviewer/logviewer.go", function: "NewLogViewer" })
}

pub async fn viewer_handlelistlog(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/logviewer/logviewer.go", function: "Viewer.HandleListLog" })
}

pub async fn viewer_handlereadlog(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/logviewer/logviewer.go", function: "Viewer.HandleReadLog" })
}

pub async fn viewer_listlogfiles(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/logviewer/logviewer.go", function: "Viewer.ListLogFiles" })
}

pub async fn viewer_loadlogfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/logviewer/logviewer.go", function: "Viewer.LoadLogFile" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
