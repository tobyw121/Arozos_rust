//! Original Go file: `mod/network/webdav/internal/xml/xml.go`
//! Package: `xml`; LOC: 1998; SHA256: `d76c5723e51f659b591892a9b4d21b4e6c0168aa70f0f44708b3cbb3c777a59c`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/xml.go", package: "xml", go_loc: 1998, functions: 46, types: 12, sha256: "d76c5723e51f659b591892a9b4d21b4e6c0168aa70f0f44708b3cbb3c777a59c" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "bytes",
    "errors",
    "fmt",
    "io",
    "strconv",
    "strings",
    "unicode",
    "unicode/utf8",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SyntaxError", "struct", 29),
    ("Name", "struct", 47),
    ("Attr", "struct", 57),
    ("Token", "interface", 64),
    ("StartElement", "struct", 67),
    ("EndElement", "struct", 109),
    ("CharData", "[", 116),
    ("Comment", "[", 128),
    ("ProcInst", "struct", 133),
    ("Directive", "[", 145),
    ("Decoder", "struct", 168),
    ("stack", "struct", 366),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Error", "e *SyntaxError", 34),
    ("isNamespace", "name Name", 52),
    ("Copy", "e StartElement", 72),
    ("End", "e StartElement", 80),
    ("setDefaultNamespace", "e *StartElement", 86),
    ("makeCopy", "", 118),
    ("Copy", "c CharData", 124),
    ("Copy", "c Comment", 130),
    ("Copy", "p ProcInst", 138),
    ("Copy", "d Directive", 147),
    ("CopyToken", "", 150),
    ("NewDecoder", "", 239),
    ("Token", "d *Decoder", 272),
    ("translate", "d *Decoder", 332),
    ("switchToReader", "d *Decoder", 350),
    ("push", "d *Decoder", 379),
    ("pop", "d *Decoder", 392),
    ("pushEOF", "d *Decoder", 405),
    ("popEOF", "d *Decoder", 431),
    ("pushElement", "d *Decoder", 440),
    ("pushNs", "d *Decoder", 447),
    ("syntaxError", "d *Decoder", 455),
    ("popElement", "d *Decoder", 465),
    ("autoClose", "d *Decoder", 503),
    ("RawToken", "d *Decoder", 526),
    ("rawToken", "d *Decoder", 533),
    ("attrval", "d *Decoder", 832),
    ("space", "d *Decoder", 867),
    ("getc", "d *Decoder", 886),
    ("InputOffset", "d *Decoder", 912),
    ("savedOffset", "d *Decoder", 918),
    ("mustgetc", "d *Decoder", 930),
    ("ungetc", "d *Decoder", 940),
    ("text", "d *Decoder", 960),
    ("isInCharacterRange", "", 1128),
    ("nsname", "d *Decoder", 1139),
    ("name", "d *Decoder", 1157),
    ("readName", "d *Decoder", 1175),
    ("isNameByte", "", 1199),
    ("isName", "", 1206),
    ("isNameString", "", 1230),
    ("EscapeText", "", 1876),
    ("escapeText", "", 1883),
    ("EscapeString", "p *printer", 1932),
    ("Escape", "", 1972),
    ("procInst", "", 1978),
];

pub async fn syntaxerror_error(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "SyntaxError.Error" })
}

pub async fn name_isnamespace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Name.isNamespace" })
}

pub async fn startelement_copy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "StartElement.Copy" })
}

pub async fn startelement_end(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "StartElement.End" })
}

pub async fn startelement_setdefaultnamespace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "StartElement.setDefaultNamespace" })
}

pub async fn makecopy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "makeCopy" })
}

pub async fn chardata_copy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "CharData.Copy" })
}

pub async fn comment_copy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Comment.Copy" })
}

pub async fn procinst_copy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "ProcInst.Copy" })
}

pub async fn directive_copy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Directive.Copy" })
}

pub async fn copytoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "CopyToken" })
}

pub async fn newdecoder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "NewDecoder" })
}

pub async fn decoder_token(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.Token" })
}

pub async fn decoder_translate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.translate" })
}

pub async fn decoder_switchtoreader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.switchToReader" })
}

pub async fn decoder_push(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.push" })
}

pub async fn decoder_pop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.pop" })
}

pub async fn decoder_pusheof(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.pushEOF" })
}

pub async fn decoder_popeof(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.popEOF" })
}

pub async fn decoder_pushelement(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.pushElement" })
}

pub async fn decoder_pushns(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.pushNs" })
}

pub async fn decoder_syntaxerror(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.syntaxError" })
}

pub async fn decoder_popelement(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.popElement" })
}

pub async fn decoder_autoclose(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.autoClose" })
}

pub async fn decoder_rawtoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.RawToken" })
}

pub async fn decoder_rawtoken_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.rawToken" })
}

pub async fn decoder_attrval(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.attrval" })
}

pub async fn decoder_space(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.space" })
}

pub async fn decoder_getc(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.getc" })
}

pub async fn decoder_inputoffset(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.InputOffset" })
}

pub async fn decoder_savedoffset(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.savedOffset" })
}

pub async fn decoder_mustgetc(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.mustgetc" })
}

pub async fn decoder_ungetc(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.ungetc" })
}

pub async fn decoder_text(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.text" })
}

pub async fn isincharacterrange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "isInCharacterRange" })
}

pub async fn decoder_nsname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.nsname" })
}

pub async fn decoder_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.name" })
}

pub async fn decoder_readname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Decoder.readName" })
}

pub async fn isnamebyte(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "isNameByte" })
}

pub async fn isname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "isName" })
}

pub async fn isnamestring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "isNameString" })
}

pub async fn escapetext(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "EscapeText" })
}

pub async fn escapetext_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "escapeText" })
}

pub async fn printer_escapestring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "printer.EscapeString" })
}

pub async fn escape(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "Escape" })
}

pub async fn procinst(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/xml.go", function: "procInst" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
