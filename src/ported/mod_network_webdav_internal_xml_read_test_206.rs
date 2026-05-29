//! Original Go file: `mod/network/webdav/internal/xml/read_test.go`
//! Package: `xml`; LOC: 744; SHA256: `572e83b5ca477df875e8dc8aaaeec1d4d759ccbed764b928acfa0c4b56c12e04`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/read_test.go", package: "xml", go_loc: 744, functions: 15, types: 27, sha256: "572e83b5ca477df875e8dc8aaaeec1d4d759ccbed764b928acfa0c4b56c12e04" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "fmt",
    "io",
    "reflect",
    "strings",
    "testing",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Feed", "struct", 85),
    ("Entry", "struct", 95),
    ("Link", "struct", 104),
    ("Person", "struct", 109),
    ("Text", "struct", 116),
    ("PathTestItem", "struct", 241),
    ("PathTestA", "struct", 245),
    ("PathTestB", "struct", 250),
    ("PathTestC", "struct", 255),
    ("PathTestSet", "struct", 261),
    ("PathTestD", "struct", 265),
    ("PathTestE", "struct", 270),
    ("BadPathTestA", "struct", 295),
    ("BadPathTestB", "struct", 301),
    ("BadPathTestC", "struct", 307),
    ("BadPathTestD", "struct", 312),
    ("BadPathEmbeddedA", "struct", 317),
    ("BadPathEmbeddedB", "struct", 321),
    ("TestThree", "struct", 348),
    ("Tables", "struct", 407),
    ("TableAttrs", "struct", 530),
    ("TAttr", "struct", 534),
    ("MyCharData", "struct", 656),
    ("MyAttr", "struct", 682),
    ("MyStruct", "struct", 693),
    ("Pea", "struct", 719),
    ("Pod", "struct", 723),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestUnmarshalFeed", "", 19),
    ("TestUnmarshalPaths", "", 283),
    ("TestUnmarshalBadPaths", "", 334),
    ("TestUnmarshalWithoutNameType", "", 353),
    ("TestUnmarshalAttr", "", 363),
    ("TestUnmarshalNS", "", 467),
    ("TestRoundTrip", "", 489),
    ("TestMarshalNS", "", 517),
    ("TestUnmarshalNSAttr", "", 612),
    ("TestMarshalNSAttr", "", 634),
    ("UnmarshalXML", "m *MyCharData", 660),
    ("UnmarshalXMLAttr", "m *MyCharData", 678),
    ("UnmarshalXMLAttr", "m *MyAttr", 686),
    ("TestUnmarshaler", "", 701),
    ("TestUnmarshalIntoInterface", "", 728),
];

pub async fn testunmarshalfeed(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalFeed" })
}

pub async fn testunmarshalpaths(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalPaths" })
}

pub async fn testunmarshalbadpaths(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalBadPaths" })
}

pub async fn testunmarshalwithoutnametype(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalWithoutNameType" })
}

pub async fn testunmarshalattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalAttr" })
}

pub async fn testunmarshalns(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalNS" })
}

pub async fn testroundtrip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestRoundTrip" })
}

pub async fn testmarshalns(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestMarshalNS" })
}

pub async fn testunmarshalnsattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalNSAttr" })
}

pub async fn testmarshalnsattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestMarshalNSAttr" })
}

pub async fn mychardata_unmarshalxml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "MyCharData.UnmarshalXML" })
}

pub async fn mychardata_unmarshalxmlattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "MyCharData.UnmarshalXMLAttr" })
}

pub async fn myattr_unmarshalxmlattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "MyAttr.UnmarshalXMLAttr" })
}

pub async fn testunmarshaler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshaler" })
}

pub async fn testunmarshalintointerface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/read_test.go", function: "TestUnmarshalIntoInterface" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
