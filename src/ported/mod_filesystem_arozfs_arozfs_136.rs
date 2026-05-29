//! Original Go file: `mod/filesystem/arozfs/arozfs.go`
//! Package: `arozfs`; LOC: 186; SHA256: `6534453c64e86556004abcf3102434b42b4a4206f335fd54ed8c846227d381d2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/arozfs/arozfs.go", package: "arozfs", go_loc: 186, functions: 9, types: 2, sha256: "6534453c64e86556004abcf3102434b42b4a4206f335fd54ed8c846227d381d2" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "io",
    "io/fs",
    "path/filepath",
    "regexp",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("File", "interface", 18),
    ("ShortcutData", "struct", 39),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewRedirectionError", "", 75),
    ("IsNetworkDrive", "", 80),
    ("GetSupportedFileSystemTypes", "", 89),
    ("GenericVirtualPathToRealPathTranslator", "", 98),
    ("GenericRealPathToVirtualPathTranslator", "", 120),
    ("GenericPathFilter", "", 143),
    ("FilterIllegalCharInFilename", "", 155),
    ("ToSlash", "", 164),
    ("Base", "", 168),
];

pub async fn newredirectionerror(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "NewRedirectionError" })
}

pub async fn isnetworkdrive(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "IsNetworkDrive" })
}

pub async fn getsupportedfilesystemtypes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "GetSupportedFileSystemTypes" })
}

pub async fn genericvirtualpathtorealpathtranslator(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "GenericVirtualPathToRealPathTranslator" })
}

pub async fn genericrealpathtovirtualpathtranslator(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "GenericRealPathToVirtualPathTranslator" })
}

pub async fn genericpathfilter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "GenericPathFilter" })
}

pub async fn filterillegalcharinfilename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "FilterIllegalCharInFilename" })
}

pub async fn toslash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "ToSlash" })
}

pub async fn base(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/arozfs/arozfs.go", function: "Base" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
