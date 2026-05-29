//! Original Go file: `mod/disk/raid/raid.go`
//! Package: `raid`; LOC: 83; SHA256: `b5cd301b3e4b925e2bd409383c2995622b5ae3028f766a963bb772d306dd0234`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/raid/raid.go", package: "raid", go_loc: 83, functions: 3, types: 2, sha256: "b5cd301b3e4b925e2bd409383c2995622b5ae3028f766a963bb772d306dd0234" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/apt",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/utils",
    "os",
    "os/exec",
    "path/filepath",
    "runtime",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Options", "struct", 21),
    ("Manager", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewRaidManager", "", 30),
    ("CreateVirtualPartition", "", 47),
    ("FormatVirtualPartition", "", 62),
];

pub async fn newraidmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid.go", function: "NewRaidManager" })
}

pub async fn createvirtualpartition(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid.go", function: "CreateVirtualPartition" })
}

pub async fn formatvirtualpartition(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/raid/raid.go", function: "FormatVirtualPartition" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
