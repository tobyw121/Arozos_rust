//! Original Go file: `mod/filesystem/metadata/psd.go`
//! Package: `metadata`; LOC: 82; SHA256: `d8d30663b69f0bbfe2b7d21ccdc8f8ea9f14ee16d96b510ddc97dd5722d47eab`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/psd.go", package: "metadata", go_loc: 82, functions: 1, types: 0, sha256: "d8d30663b69f0bbfe2b7d21ccdc8f8ea9f14ee16d96b510ddc97dd5722d47eab" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/nfnt/resize",
    "github.com/oliamb/cutter",
    "github.com/oov/psd",
    "image",
    "image/jpeg",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForPSD", "", 16),
];

pub async fn generatethumbnailforpsd(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/psd.go", function: "generateThumbnailForPSD" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
