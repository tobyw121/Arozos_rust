//! Original Go file: `storage_reload_signal.go`
//! Package: `main`; LOC: 49; SHA256: `d7a596420515d4f2c224e6a5660de4cc5d9ae6e315444836384b3110f386491f`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "storage_reload_signal.go", package: "main", go_loc: 49, functions: 1, types: 0, sha256: "d7a596420515d4f2c224e6a5660de4cc5d9ae6e315444836384b3110f386491f" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/storage",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ReloadAllStoragePools", "", 14),
];

pub async fn reloadallstoragepools(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "storage_reload_signal.go", function: "ReloadAllStoragePools" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
