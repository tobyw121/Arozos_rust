//! Original Go file: `mod/permission/static.go`
//! Package: `permission`; LOC: 16; SHA256: `d5a17edda93075776d1d212398688e128a78506e5ff8cabed50f06186a986361`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/permission/static.go", package: "permission", go_loc: 16, functions: 1, types: 0, sha256: "d5a17edda93075776d1d212398688e128a78506e5ff8cabed50f06186a986361" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetLargestStorageQuotaFromGroups", "", 4),
];

pub async fn getlargeststoragequotafromgroups(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/permission/static.go", function: "GetLargestStorageQuotaFromGroups" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
