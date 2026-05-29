//! Original Go file: `mod/filesystem/fileOpr.go`
//! Package: `filesystem`; LOC: 997; SHA256: `41fe39dca5dafb35a8ea423de8470c5f841640f14ade50facbe497890457a1ee`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/fileOpr.go", package: "filesystem", go_loc: 997, functions: 17, types: 0, sha256: "41fe39dca5dafb35a8ea423de8470c5f841640f14ade50facbe497890457a1ee" };

pub const GO_IMPORTS: &[&str] = &[
    "archive/tar",
    "archive/zip",
    "compress/flate",
    "compress/gzip",
    "errors",
    "fmt",
    "github.com/mholt/archiver/v3",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/hidden",
    "io",
    "log",
    "os",
    "path/filepath",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ZipFile", "", 36),
    ("Unzip", "", 50),
    ("ArozUnzipFileWithProgress", "", 92),
    ("ArozZipFileWithProgress", "", 188),
    ("ArozZipFile", "", 334),
    ("ArozZipFileWithCompressionLevel", "", 341),
    ("insideHiddenFolder", "", 514),
    ("ViewZipFile", "", 523),
    ("FileCopy", "", 534),
    ("FileMove", "", 624),
    ("CopyDir", "", 763),
    ("dirCopy", "", 768),
    ("BasicFileCopy", "", 842),
    ("BufferedLargeFileCopy", "", 870),
    ("IsDir", "", 915),
    ("ExtractTarGzipFile", "", 934),
    ("ExtractTarGzipByStream", "", 947),
];

pub async fn zipfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ZipFile" })
}

pub async fn unzip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "Unzip" })
}

pub async fn arozunzipfilewithprogress(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ArozUnzipFileWithProgress" })
}

pub async fn arozzipfilewithprogress(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ArozZipFileWithProgress" })
}

pub async fn arozzipfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ArozZipFile" })
}

pub async fn arozzipfilewithcompressionlevel(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ArozZipFileWithCompressionLevel" })
}

pub async fn insidehiddenfolder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "insideHiddenFolder" })
}

pub async fn viewzipfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ViewZipFile" })
}

pub async fn filecopy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "FileCopy" })
}

pub async fn filemove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "FileMove" })
}

pub async fn copydir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "CopyDir" })
}

pub async fn dircopy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "dirCopy" })
}

pub async fn basicfilecopy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "BasicFileCopy" })
}

pub async fn bufferedlargefilecopy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "BufferedLargeFileCopy" })
}

pub async fn isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "IsDir" })
}

pub async fn extracttargzipfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ExtractTarGzipFile" })
}

pub async fn extracttargzipbystream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fileOpr.go", function: "ExtractTarGzipByStream" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
