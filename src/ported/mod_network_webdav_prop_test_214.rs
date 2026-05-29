//! Original Go file: `mod/network/webdav/prop_test.go`
//! Package: `webdav`; LOC: 716; SHA256: `fafbc38922b001bc9c90cfec6769df9cb95529d910f66061d46b43cf6785a0e4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/prop_test.go", package: "webdav", go_loc: 716, functions: 22, types: 7, sha256: "fafbc38922b001bc9c90cfec6769df9cb95529d910f66061d46b43cf6785a0e4" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "encoding/xml",
    "fmt",
    "net/http",
    "os",
    "reflect",
    "regexp",
    "sort",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("byXMLName", "[", 572),
    ("byPropname", "[", 578),
    ("byStatus", "[", 584),
    ("noDeadPropsFS", "struct", 590),
    ("noDeadPropsFile", "struct", 604),
    ("overrideContentType", "struct", 615),
    ("overrideETag", "struct", 666),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestMemPS", "", 19),
    ("cmpXMLName", "", 565),
    ("Len", "b byXMLName", 574),
    ("Swap", "b byXMLName", 575),
    ("Less", "b byXMLName", 576),
    ("Len", "b byPropname", 580),
    ("Swap", "b byPropname", 581),
    ("Less", "b byPropname", 582),
    ("Len", "b byStatus", 586),
    ("Swap", "b byStatus", 587),
    ("Less", "b byStatus", 588),
    ("OpenFile", "fs noDeadPropsFS", 594),
    ("Close", "f noDeadPropsFile", 608),
    ("Read", "f noDeadPropsFile", 609),
    ("Readdir", "f noDeadPropsFile", 610),
    ("Seek", "f noDeadPropsFile", 611),
    ("Stat", "f noDeadPropsFile", 612),
    ("Write", "f noDeadPropsFile", 613),
    ("ContentType", "o *overrideContentType", 621),
    ("TestFindContentTypeOverride", "", 625),
    ("ETag", "o *overrideETag", 672),
    ("TestFindETagOverride", "", 676),
];

pub async fn testmemps(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "TestMemPS" })
}

pub async fn cmpxmlname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "cmpXMLName" })
}

pub async fn byxmlname_len(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byXMLName.Len" })
}

pub async fn byxmlname_swap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byXMLName.Swap" })
}

pub async fn byxmlname_less(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byXMLName.Less" })
}

pub async fn bypropname_len(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byPropname.Len" })
}

pub async fn bypropname_swap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byPropname.Swap" })
}

pub async fn bypropname_less(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byPropname.Less" })
}

pub async fn bystatus_len(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byStatus.Len" })
}

pub async fn bystatus_swap(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byStatus.Swap" })
}

pub async fn bystatus_less(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "byStatus.Less" })
}

pub async fn nodeadpropsfs_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFS.OpenFile" })
}

pub async fn nodeadpropsfile_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFile.Close" })
}

pub async fn nodeadpropsfile_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFile.Read" })
}

pub async fn nodeadpropsfile_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFile.Readdir" })
}

pub async fn nodeadpropsfile_seek(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFile.Seek" })
}

pub async fn nodeadpropsfile_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFile.Stat" })
}

pub async fn nodeadpropsfile_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "noDeadPropsFile.Write" })
}

pub async fn overridecontenttype_contenttype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "overrideContentType.ContentType" })
}

pub async fn testfindcontenttypeoverride(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "TestFindContentTypeOverride" })
}

pub async fn overrideetag_etag(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "overrideETag.ETag" })
}

pub async fn testfindetagoverride(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/prop_test.go", function: "TestFindETagOverride" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
