//! Original Go file: `mod/network/webdav/internal/xml/marshal_test.go`
//! Package: `xml`; LOC: 1939; SHA256: `af57832bfcb8126a19d5a6f4164788c354f79092e6c236adb517f55937e3b99f`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/marshal_test.go", package: "xml", go_loc: 1939, functions: 22, types: 67, sha256: "af57832bfcb8126a19d5a6f4164788c354f79092e6c236adb517f55937e3b99f" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "errors",
    "fmt",
    "io",
    "reflect",
    "strconv",
    "strings",
    "sync",
    "testing",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("DriveType", "i", 20),
    ("Passenger", "struct", 27),
    ("Ship", "struct", 32),
    ("NamedType", "s", 43),
    ("Port", "struct", 45),
    ("Domain", "struct", 52),
    ("Book", "struct", 59),
    ("Event", "struct", 64),
    ("Movie", "struct", 69),
    ("Pi", "struct", 74),
    ("Universe", "struct", 79),
    ("Particle", "struct", 84),
    ("Departure", "struct", 89),
    ("SecretAgent", "struct", 94),
    ("NestedItems", "struct", 101),
    ("NestedOrder", "struct", 107),
    ("MixedNested", "struct", 114),
    ("NilTest", "struct", 122),
    ("Service", "struct", 128),
    ("EmbedA", "struct", 138),
    ("EmbedB", "struct", 144),
    ("EmbedC", "struct", 149),
    ("NameCasing", "struct", 156),
    ("NamePrecedence", "struct", 164),
    ("XMLNameWithTag", "struct", 172),
    ("XMLNameWithNSTag", "struct", 177),
    ("XMLNameWithoutTag", "struct", 182),
    ("NameInField", "struct", 187),
    ("AttrTest", "struct", 191),
    ("OmitAttrTest", "struct", 201),
    ("OmitFieldTest", "struct", 211),
    ("AnyTest", "struct", 222),
    ("AnyOmitTest", "struct", 228),
    ("AnySliceTest", "struct", 234),
    ("AnyHolder", "struct", 240),
    ("RecurseA", "struct", 245),
    ("RecurseB", "struct", 250),
    ("PresenceTest", "struct", 255),
    ("IgnoreTest", "struct", 259),
    ("MyBytes", "[", 263),
    ("Data", "struct", 265),
    ("Plain", "struct", 271),
    ("MyInt", "i", 275),
    ("EmbedInt", "struct", 277),
    ("Strings", "struct", 281),
    ("PointerFieldsTest", "struct", 285),
    ("ChardataEmptyTest", "struct", 293),
    ("MyMarshalerTest", "struct", 298),
    ("MyMarshalerAttrTest", "struct", 310),
    ("MyMarshalerValueAttrTest", "struct", 318),
    ("MarshalerStruct", "struct", 326),
    ("MarshalerValueStruct", "struct", 330),
    ("InnerStruct", "struct", 334),
    ("OuterStruct", "struct", 338),
    ("OuterNamedStruct", "struct", 343),
    ("OuterNamedOrderedStruct", "struct", 349),
    ("OuterOuterStruct", "struct", 355),
    ("NestedAndChardata", "struct", 359),
    ("NestedAndComment", "struct", 364),
    ("XMLNSFieldStruct", "struct", 369),
    ("NamedXMLNSFieldStruct", "struct", 374),
    ("XMLNSFieldStructWithOmitEmpty", "struct", 380),
    ("NamedXMLNSFieldStructWithEmptyNamespace", "struct", 385),
    ("RecursiveXMLNSFieldStruct", "struct", 391),
    ("AttrParent", "struct", 1119),
    ("BadAttr", "struct", 1123),
    ("limitedBytesWriter", "struct", 1243),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("MarshalXML", "m *MyMarshalerTest", 303),
    ("MarshalXMLAttr", "m *MyMarshalerAttrTest", 314),
    ("MarshalXMLAttr", "m MyMarshalerValueAttrTest", 322),
    ("ifaceptr", "", 397),
    ("TestMarshal", "", 1099),
    ("TestMarshalErrors", "", 1183),
    ("TestUnmarshal", "", 1202),
    ("TestMarshalIndent", "", 1230),
    ("Write", "lw *limitedBytesWriter", 1248),
    ("TestMarshalWriteErrors", "", 1264),
    ("TestMarshalWriteIOErrors", "", 1292),
    ("TestMarshalFlush", "", 1302),
    ("TestEncodeElement", "", 1395),
    ("BenchmarkMarshal", "", 1413),
    ("BenchmarkUnmarshal", "", 1420),
    ("TestStructPointerMarshal", "", 1429),
    ("TestEncodeToken", "", 1789),
    ("TestProcInstEncodeToken", "", 1827),
    ("TestDecodeEncode", "", 1844),
    ("TestRace9796", "", 1862),
    ("TestIsValidDirective", "", 1878),
    ("TestSimpleUseOfEncodeToken", "", 1911),
];

pub async fn mymarshalertest_marshalxml(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "MyMarshalerTest.MarshalXML" })
}

pub async fn mymarshalerattrtest_marshalxmlattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "MyMarshalerAttrTest.MarshalXMLAttr" })
}

pub async fn mymarshalervalueattrtest_marshalxmlattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "MyMarshalerValueAttrTest.MarshalXMLAttr" })
}

pub async fn ifaceptr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "ifaceptr" })
}

pub async fn testmarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestMarshal" })
}

pub async fn testmarshalerrors(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestMarshalErrors" })
}

pub async fn testunmarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestUnmarshal" })
}

pub async fn testmarshalindent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestMarshalIndent" })
}

pub async fn limitedbyteswriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "limitedBytesWriter.Write" })
}

pub async fn testmarshalwriteerrors(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestMarshalWriteErrors" })
}

pub async fn testmarshalwriteioerrors(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestMarshalWriteIOErrors" })
}

pub async fn testmarshalflush(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestMarshalFlush" })
}

pub async fn testencodeelement(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestEncodeElement" })
}

pub async fn benchmarkmarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "BenchmarkMarshal" })
}

pub async fn benchmarkunmarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "BenchmarkUnmarshal" })
}

pub async fn teststructpointermarshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestStructPointerMarshal" })
}

pub async fn testencodetoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestEncodeToken" })
}

pub async fn testprocinstencodetoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestProcInstEncodeToken" })
}

pub async fn testdecodeencode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestDecodeEncode" })
}

pub async fn testrace9796(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestRace9796" })
}

pub async fn testisvaliddirective(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestIsValidDirective" })
}

pub async fn testsimpleuseofencodetoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal_test.go", function: "TestSimpleUseOfEncodeToken" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
