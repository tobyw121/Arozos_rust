//! Original Go file: `disk.go`
//! Package: `main`; LOC: 234; SHA256: `48f018ca03dde77507fb6dda5baf70486eb7815b458a579ee2fd5ce5d90cc5d9`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "disk.go", package: "main", go_loc: 234, functions: 2, types: 0, sha256: "48f018ca03dde77507fb6dda5baf70486eb7815b458a579ee2fd5ce5d90cc5d9" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/disk/diskcapacity",
    "imuslab.com/arozos/mod/disk/diskmg",
    "imuslab.com/arozos/mod/disk/diskspace",
    "imuslab.com/arozos/mod/disk/raid",
    "imuslab.com/arozos/mod/disk/smart",
    "imuslab.com/arozos/mod/disk/sortfile",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("RAIDServiceInit", "", 22),
    ("DiskServiceInit", "", 53),
];

pub async fn raidserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk.go", function: "RAIDServiceInit" })
}

pub async fn diskserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "disk.go", function: "DiskServiceInit" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
