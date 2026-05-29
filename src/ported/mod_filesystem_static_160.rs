//! Original Go file: `mod/filesystem/static.go`
//! Package: `filesystem`; LOC: 493; SHA256: `636695818081aac76bdc5086a0193350b3af9297e37a67eee5ca8a88509469e4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/static.go", package: "filesystem", go_loc: 493, functions: 20, types: 4, sha256: "636695818081aac76bdc5086a0193350b3af9297e37a67eee5ca8a88509469e4" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/md5",
    "crypto/sha256",
    "encoding/hex",
    "errors",
    "fmt",
    "github.com/gabriel-vasile/mimetype",
    "imuslab.com/arozos/mod/apt",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/shortcut",
    "io",
    "log",
    "mime",
    "net/url",
    "os",
    "os/exec",
    "path/filepath",
    "runtime",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FileData", "struct", 37),
    ("TrashedFile", "struct", 49),
    ("FileProperties", "struct", 61),
    ("EmptyHierarchySpecificConfig", "struct", 80),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ResolveVrootPath", "e EmptyHierarchySpecificConfig", 84),
    ("ResolveRealPath", "e EmptyHierarchySpecificConfig", 87),
    ("MatchingFileSystem", "", 96),
    ("GetIDFromVirtualPath", "", 101),
    ("GetFileDataFromPath", "", 120),
    ("CheckMounted", "", 152),
    ("MountDevice", "", 177),
    ("GetFileSize", "", 223),
    ("IsInsideHiddenFolder", "", 232),
    ("WGlob", "", 248),
    ("GetDirectorySizeNative", "", 280),
    ("GetDirctorySize", "", 328),
    ("GetFileDisplaySize", "", 358),
    ("DecodeURI", "", 399),
    ("GetMime", "", 406),
    ("GetModTime", "", 414),
    ("UnderTheSameRoot", "", 427),
    ("GetPhysicalRootFromPath", "", 447),
    ("GetFileSHA256Sum", "", 465),
    ("GetFileMD5Sum", "", 480),
];

pub async fn emptyhierarchyspecificconfig_resolvevrootpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "EmptyHierarchySpecificConfig.ResolveVrootPath" })
}

pub async fn emptyhierarchyspecificconfig_resolverealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "EmptyHierarchySpecificConfig.ResolveRealPath" })
}

pub async fn matchingfilesystem(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "MatchingFileSystem" })
}

pub async fn getidfromvirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetIDFromVirtualPath" })
}

pub async fn getfiledatafrompath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetFileDataFromPath" })
}

pub async fn checkmounted(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "CheckMounted" })
}

pub async fn mountdevice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "MountDevice" })
}

pub async fn getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetFileSize" })
}

pub async fn isinsidehiddenfolder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "IsInsideHiddenFolder" })
}

pub async fn wglob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "WGlob" })
}

pub async fn getdirectorysizenative(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetDirectorySizeNative" })
}

pub async fn getdirctorysize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetDirctorySize" })
}

pub async fn getfiledisplaysize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetFileDisplaySize" })
}

pub async fn decodeuri(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "DecodeURI" })
}

pub async fn getmime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetMime" })
}

pub async fn getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetModTime" })
}

pub async fn underthesameroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "UnderTheSameRoot" })
}

pub async fn getphysicalrootfrompath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetPhysicalRootFromPath" })
}

pub async fn getfilesha256sum(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetFileSHA256Sum" })
}

pub async fn getfilemd5sum(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/static.go", function: "GetFileMD5Sum" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
