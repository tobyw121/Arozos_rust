//! Original Go file: `mod/filesystem/metadata/model.go`
//! Package: `metadata`; LOC: 61; SHA256: `0676330a98efcde401f190129dad91fc775d1e78a5de0ad62915d751486894f4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/model.go", package: "metadata", go_loc: 61, functions: 1, types: 0, sha256: "0676330a98efcde401f190129dad91fc775d1e78a5de0ad62915d751486894f4" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "image/jpeg",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/renderer",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForModel", "", 12),
];

pub async fn generatethumbnailformodel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/model.go", function: "generateThumbnailForModel" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
