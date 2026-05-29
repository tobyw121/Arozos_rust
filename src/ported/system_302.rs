//! Original Go file: `system.go`
//! Package: `main`; LOC: 211; SHA256: `c122972894ed1276bd5920d00051105576d6bb1f5c19db7d124d564e69ff03ff`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "system.go", package: "main", go_loc: 211, functions: 9, types: 0, sha256: "c122972894ed1276bd5920d00051105576d6bb1f5c19db7d124d564e69ff03ff" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SystemIDInit", "", 22),
    ("systemIdHandlePing", "", 86),
    ("systemIdGenerateSystemUUID", "", 98),
    ("systemIdGetSystemUUID", "", 120),
    ("systemHandleListLicense", "", 130),
    ("systemIdHandleRequest", "", 144),
    ("systemIdResponseBetaScan", "", 176),
    ("systemIdServeVersonNumber", "", 194),
    ("systemIdGetDriveStates", "", 202),
];

pub async fn systemidinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "SystemIDInit" })
}

pub async fn systemidhandleping(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdHandlePing" })
}

pub async fn systemidgeneratesystemuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdGenerateSystemUUID" })
}

pub async fn systemidgetsystemuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdGetSystemUUID" })
}

pub async fn systemhandlelistlicense(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemHandleListLicense" })
}

pub async fn systemidhandlerequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdHandleRequest" })
}

pub async fn systemidresponsebetascan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdResponseBetaScan" })
}

pub async fn systemidserveversonnumber(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdServeVersonNumber" })
}

pub async fn systemidgetdrivestates(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.go", function: "systemIdGetDriveStates" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
