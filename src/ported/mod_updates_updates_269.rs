//! Original Go file: `mod/updates/updates.go`
//! Package: `updates`; LOC: 191; SHA256: `523519e82a7902c9ef78ace61e5bf2ea087d98f41122b90404385d72c4788ac8`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/updates/updates.go", package: "updates", go_loc: 191, functions: 4, types: 0, sha256: "523519e82a7902c9ef78ace61e5bf2ea087d98f41122b90404385d72c4788ac8" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "io",
    "net/http",
    "os",
    "path/filepath",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("DownloadUpdatesFromURL", "", 13),
    ("GetUpdateSizes", "", 150),
    ("GetLauncherVersion", "", 163),
    ("CheckLauncherPortResponsive", "", 181),
];

pub async fn downloadupdatesfromurl(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/updates.go", function: "DownloadUpdatesFromURL" })
}

pub async fn getupdatesizes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/updates.go", function: "GetUpdateSizes" })
}

pub async fn getlauncherversion(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/updates.go", function: "GetLauncherVersion" })
}

pub async fn checklauncherportresponsive(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/updates.go", function: "CheckLauncherPortResponsive" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
