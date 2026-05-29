//! Original Go file: `mod/network/webdav/internal/xml/xml_test.go`
//! Package: `xml`; LOC: 752; SHA256: `c9c9a78302c5bd49f59621d224154984a2f7c4d3b06733d92cda2c979bce1fea`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/xml_test.go", package: "xml", go_loc: 752, functions: 27, types: 5, sha256: "c9c9a78302c5bd49f59621d224154984a2f7c4d3b06733d92cda2c979bce1fea" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "fmt",
    "io",
    "reflect",
    "strings",
    "testing",
    "unicode/utf8",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("downCaser", "struct", 231),
    ("allScalars", "struct", 402),
    ("item", "struct", 483),
    ("procInstEncodingTest", "struct", 655),
    ("errWriter", "struct", 715),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestRawToken", "", 170),
    ("TestNonStrictRawToken", "", 225),
    ("ReadByte", "d *downCaser", 236),
    ("Read", "d *downCaser", 244),
    ("TestRawTokenAltEncoding", "", 249),
    ("TestRawTokenAltEncodingNoConverter", "", 260),
    ("testRawToken", "", 283),
    ("TestNestedDirectives", "", 361),
    ("TestToken", "", 375),
    ("TestSyntax", "", 390),
    ("TestAllScalars", "", 471),
    ("TestIssue569", "", 487),
    ("TestUnquotedAttrs", "", 497),
    ("TestValuelessAttrs", "", 517),
    ("TestCopyTokenCharData", "", 544),
    ("TestCopyTokenStartElement", "", 557),
    ("TestSyntaxErrorLineNum", "", 573),
    ("TestTrailingRawToken", "", 588),
    ("TestTrailingToken", "", 599),
    ("TestEntityInsideCDATA", "", 610),
    ("TestDisallowedCharacters", "", 636),
    ("TestProcInstEncoding", "", 670),
    ("TestDirectivesWithComments", "", 700),
    ("Write", "errWriter", 717),
    ("TestEscapeTextIOErrors", "", 719),
    ("TestEscapeTextInvalidChar", "", 728),
    ("TestIssue5880", "", 743),
];

pub async fn testrawtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestRawToken" })
}

pub async fn testnonstrictrawtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestNonStrictRawToken" })
}

pub async fn downcaser_readbyte(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "downCaser.ReadByte" })
}

pub async fn downcaser_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "downCaser.Read" })
}

pub async fn testrawtokenaltencoding(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestRawTokenAltEncoding" })
}

pub async fn testrawtokenaltencodingnoconverter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestRawTokenAltEncodingNoConverter" })
}

pub async fn testrawtoken_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "testRawToken" })
}

pub async fn testnesteddirectives(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestNestedDirectives" })
}

pub async fn testtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestToken" })
}

pub async fn testsyntax(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestSyntax" })
}

pub async fn testallscalars(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestAllScalars" })
}

pub async fn testissue569(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestIssue569" })
}

pub async fn testunquotedattrs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestUnquotedAttrs" })
}

pub async fn testvaluelessattrs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestValuelessAttrs" })
}

pub async fn testcopytokenchardata(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestCopyTokenCharData" })
}

pub async fn testcopytokenstartelement(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestCopyTokenStartElement" })
}

pub async fn testsyntaxerrorlinenum(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestSyntaxErrorLineNum" })
}

pub async fn testtrailingrawtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestTrailingRawToken" })
}

pub async fn testtrailingtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestTrailingToken" })
}

pub async fn testentityinsidecdata(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestEntityInsideCDATA" })
}

pub async fn testdisallowedcharacters(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestDisallowedCharacters" })
}

pub async fn testprocinstencoding(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestProcInstEncoding" })
}

pub async fn testdirectiveswithcomments(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestDirectivesWithComments" })
}

pub async fn errwriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "errWriter.Write" })
}

pub async fn testescapetextioerrors(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestEscapeTextIOErrors" })
}

pub async fn testescapetextinvalidchar(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestEscapeTextInvalidChar" })
}

pub async fn testissue5880(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml_test.go", function: "TestIssue5880" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
