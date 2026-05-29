//! Original Go file: `mod/network/webdav/internal/xml/marshal.go`
//! Package: `xml`; LOC: 1223; SHA256: `9369f9331daf308bcc0d17b84133d0009bd74b82bcbcd6d3d89f5844018522e3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/marshal.go", package: "xml", go_loc: 1223, functions: 33, types: 7, sha256: "9369f9331daf308bcc0d17b84133d0009bd74b82bcbcd6d3d89f5844018522e3" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "bytes",
    "encoding",
    "fmt",
    "io",
    "reflect",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Marshaler", "interface", 94),
    ("MarshalerAttr", "interface", 109),
    ("Encoder", "struct", 127),
    ("printer", "struct", 302),
    ("printerPrefix", "struct", 325),
    ("parentStack", "struct", 1135),
    ("UnsupportedTypeError", "struct", 1199),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Marshal", "", 70),
    ("MarshalIndent", "", 116),
    ("NewEncoder", "", 132),
    ("Indent", "enc *Encoder", 141),
    ("Encode", "enc *Encoder", 152),
    ("EncodeElement", "enc *Encoder", 167),
    ("EncodeToken", "enc *Encoder", 201),
    ("isValidDirective", "", 258),
    ("Flush", "enc *Encoder", 298),
    ("prefixForNS", "p *printer", 331),
    ("defineNS", "p *printer", 349),
    ("createNSPrefix", "p *printer", 403),
    ("writeNamespaces", "p *printer", 460),
    ("pushPrefix", "p *printer", 482),
    ("nsForPrefix", "p *printer", 494),
    ("markPrefix", "p *printer", 503),
    ("popPrefix", "p *printer", 511),
    ("setAttrPrefix", "p *printer", 525),
    ("marshalValue", "p *printer", 556),
    ("fieldAttr", "p *printer", 722),
    ("defaultStart", "p *printer", 779),
    ("marshalInterface", "p *printer", 806),
    ("marshalTextInterface", "p *printer", 826),
    ("writeStart", "p *printer", 839),
    ("writeName", "p *printer", 894),
    ("writeEnd", "p *printer", 902),
    ("marshalSimple", "p *printer", 926),
    ("marshalStruct", "p *printer", 963),
    ("cachedWriteError", "p *printer", 1099),
    ("writeIndent", "p *printer", 1104),
    ("setParents", "s *parentStack", 1144),
    ("Error", "e *UnsupportedTypeError", 1203),
    ("isEmptyValue", "", 1207),
];

pub async fn marshal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "Marshal" })
}

pub async fn marshalindent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "MarshalIndent" })
}

pub async fn newencoder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "NewEncoder" })
}

pub async fn encoder_indent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "Encoder.Indent" })
}

pub async fn encoder_encode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "Encoder.Encode" })
}

pub async fn encoder_encodeelement(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "Encoder.EncodeElement" })
}

pub async fn encoder_encodetoken(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "Encoder.EncodeToken" })
}

pub async fn isvaliddirective(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "isValidDirective" })
}

pub async fn encoder_flush(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "Encoder.Flush" })
}

pub async fn printer_prefixforns(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.prefixForNS" })
}

pub async fn printer_definens(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.defineNS" })
}

pub async fn printer_creatensprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.createNSPrefix" })
}

pub async fn printer_writenamespaces(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.writeNamespaces" })
}

pub async fn printer_pushprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.pushPrefix" })
}

pub async fn printer_nsforprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.nsForPrefix" })
}

pub async fn printer_markprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.markPrefix" })
}

pub async fn printer_popprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.popPrefix" })
}

pub async fn printer_setattrprefix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.setAttrPrefix" })
}

pub async fn printer_marshalvalue(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.marshalValue" })
}

pub async fn printer_fieldattr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.fieldAttr" })
}

pub async fn printer_defaultstart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.defaultStart" })
}

pub async fn printer_marshalinterface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.marshalInterface" })
}

pub async fn printer_marshaltextinterface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.marshalTextInterface" })
}

pub async fn printer_writestart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.writeStart" })
}

pub async fn printer_writename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.writeName" })
}

pub async fn printer_writeend(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.writeEnd" })
}

pub async fn printer_marshalsimple(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.marshalSimple" })
}

pub async fn printer_marshalstruct(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.marshalStruct" })
}

pub async fn printer_cachedwriteerror(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.cachedWriteError" })
}

pub async fn printer_writeindent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "printer.writeIndent" })
}

pub async fn parentstack_setparents(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "parentStack.setParents" })
}

pub async fn unsupportedtypeerror_error(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "UnsupportedTypeError.Error" })
}

pub async fn isemptyvalue(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/marshal.go", function: "isEmptyValue" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
