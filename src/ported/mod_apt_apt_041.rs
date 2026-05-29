//! Original Go file: `mod/apt/apt.go`
//! Package: `apt`; LOC: 173; SHA256: `c132806f0bc7dad7b2efe08b833692a3704fe09c7c65a00551d077ddb0165f5b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/apt/apt.go", package: "apt", go_loc: 173, functions: 4, types: 1, sha256: "c132806f0bc7dad7b2efe08b833692a3704fe09c7c65a00551d077ddb0165f5b" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "log",
    "net/http",
    "os",
    "os/exec",
    "runtime",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("AptPackageManager", "struct", 20),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewPackageManager", "", 24),
    ("InstallIfNotExists", "a *AptPackageManager", 31),
    ("PackageExists", "", 71),
    ("HandlePackageListRequest", "", 108),
];

pub async fn newpackagemanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/apt/apt.go", function: "NewPackageManager" })
}

pub async fn aptpackagemanager_installifnotexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/apt/apt.go", function: "AptPackageManager.InstallIfNotExists" })
}

pub async fn packageexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/apt/apt.go", function: "PackageExists" })
}

pub async fn handlepackagelistrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/apt/apt.go", function: "HandlePackageListRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
