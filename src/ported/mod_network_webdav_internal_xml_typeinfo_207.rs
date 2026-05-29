//! Original Go file: `mod/network/webdav/internal/xml/typeinfo.go`
//! Package: `xml`; LOC: 371; SHA256: `b899c6a29eb958a215f8db3bc2435f38a938c683b9ee6c98d44a11053156bf01`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/typeinfo.go", package: "xml", go_loc: 371, functions: 7, types: 4, sha256: "b899c6a29eb958a215f8db3bc2435f38a938c683b9ee6c98d44a11053156bf01" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "reflect",
    "strings",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("typeInfo", "struct", 15),
    ("fieldInfo", "struct", 21),
    ("fieldFlags", "i", 29),
    ("TagPathError", "struct", 344),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getTypeInfo", "", 51),
    ("structFieldInfo", "", 114),
    ("lookupXMLName", "", 238),
    ("min", "", 261),
    ("addFieldInfo", "", 275),
    ("Error", "e *TagPathError", 350),
    ("value", "finfo *fieldInfo", 357),
];

pub async fn gettypeinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "getTypeInfo" })
}

pub async fn structfieldinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "structFieldInfo" })
}

pub async fn lookupxmlname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "lookupXMLName" })
}

pub async fn min(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "min" })
}

pub async fn addfieldinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "addFieldInfo" })
}

pub async fn tagpatherror_error(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "TagPathError.Error" })
}

pub async fn fieldinfo_value(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/typeinfo.go", function: "fieldInfo.value" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
