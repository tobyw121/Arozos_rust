//! Original Go file: `mod/filesystem/metadata/audio.go`
//! Package: `metadata`; LOC: 77; SHA256: `b8392f69ce9e558a294c5e247769763d85f6ddb4afbbbf0cbff0a66bfe2fdb46`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/audio.go", package: "metadata", go_loc: 77, functions: 1, types: 0, sha256: "b8392f69ce9e558a294c5e247769763d85f6ddb4afbbbf0cbff0a66bfe2fdb46" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "github.com/dhowden/tag",
    "github.com/nfnt/resize",
    "github.com/oliamb/cutter",
    "image",
    "image/jpeg",
    "imuslab.com/arozos/mod/filesystem",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForAudio", "", 15),
];

pub async fn generatethumbnailforaudio(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/audio.go", function: "generateThumbnailForAudio" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
