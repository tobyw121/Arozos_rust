//! Original Go file: `desktop.go`
//! Package: `main`; LOC: 779; SHA256: `e903918b7cf0796c868303dfeac5e3572108694dce88c96d88bb9563d86e3586`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "desktop.go", package: "main", go_loc: 779, functions: 13, types: 0, sha256: "e903918b7cf0796c868303dfeac5e3572108694dce88c96d88bb9563d86e3586" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/shortcut",
    "imuslab.com/arozos/mod/modules",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("DesktopInit", "", 23),
    ("desktop_initUserFolderStructure", "", 74),
    ("desktop_hostdetailHandler", "", 103),
    ("desktop_handleShortcutRename", "", 125),
    ("desktop_listFiles", "", 192),
    ("getDesktopLocatioFromPath", "", 307),
    ("setDesktopLocationFromPath", "", 329),
    ("delDesktopLocationFromPath", "", 363),
    ("desktop_handleUserInfo", "", 369),
    ("desktop_fileLocation_handler", "", 462),
    ("desktop_theme_handler", "", 510),
    ("desktop_preference_handler", "", 660),
    ("desktop_shortcutHandler", "", 698),
];

pub async fn desktopinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "DesktopInit" })
}

pub async fn desktop_inituserfolderstructure(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_initUserFolderStructure" })
}

pub async fn desktop_hostdetailhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_hostdetailHandler" })
}

pub async fn desktop_handleshortcutrename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_handleShortcutRename" })
}

pub async fn desktop_listfiles(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_listFiles" })
}

pub async fn getdesktoplocatiofrompath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "getDesktopLocatioFromPath" })
}

pub async fn setdesktoplocationfrompath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "setDesktopLocationFromPath" })
}

pub async fn deldesktoplocationfrompath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "delDesktopLocationFromPath" })
}

pub async fn desktop_handleuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_handleUserInfo" })
}

pub async fn desktop_filelocation_handler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_fileLocation_handler" })
}

pub async fn desktop_theme_handler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_theme_handler" })
}

pub async fn desktop_preference_handler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_preference_handler" })
}

pub async fn desktop_shortcuthandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "desktop.go", function: "desktop_shortcutHandler" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
