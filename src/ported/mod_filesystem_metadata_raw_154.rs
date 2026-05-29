//! Original Go file: `mod/filesystem/metadata/raw.go`
//! Package: `metadata`; LOC: 598; SHA256: `ec1ef991c60843925758afa88ab6bd15d498c85edb30e5301bb6c630f26a54aa`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/raw.go", package: "metadata", go_loc: 598, functions: 9, types: 0, sha256: "ec1ef991c60843925758afa88ab6bd15d498c85edb30e5301bb6c630f26a54aa" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "encoding/binary",
    "errors",
    "github.com/nfnt/resize",
    "github.com/oliamb/cutter",
    "image",
    "image/jpeg",
    "imuslab.com/arozos/mod/filesystem",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForRAW", "", 18),
    ("extractLargestJPEG", "", 135),
    ("extractJPEGFromTIFF", "", 220),
    ("extractUncompressedThumbnail", "", 269),
    ("parseTIFFIFDChain", "", 357),
    ("readIFDValue", "", 477),
    ("readIFDArray", "", 493),
    ("getSizeForType", "", 533),
    ("RenderRAWImage", "", 544),
];

pub async fn generatethumbnailforraw(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "generateThumbnailForRAW" })
}

pub async fn extractlargestjpeg(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "extractLargestJPEG" })
}

pub async fn extractjpegfromtiff(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "extractJPEGFromTIFF" })
}

pub async fn extractuncompressedthumbnail(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "extractUncompressedThumbnail" })
}

pub async fn parsetiffifdchain(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "parseTIFFIFDChain" })
}

pub async fn readifdvalue(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "readIFDValue" })
}

pub async fn readifdarray(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "readIFDArray" })
}

pub async fn getsizefortype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "getSizeForType" })
}

pub async fn renderrawimage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/raw.go", function: "RenderRAWImage" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
