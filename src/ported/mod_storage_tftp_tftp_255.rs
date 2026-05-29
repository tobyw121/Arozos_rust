//! Original Go file: `mod/storage/tftp/tftp.go`
//! Package: `tftp`; LOC: 187; SHA256: `6ce53106ad22380201d29be4c08c55df37d933123ba4ca0a03655f7b8779b02a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/tftp/tftp.go", package: "tftp", go_loc: 187, functions: 5, types: 2, sha256: "6ce53106ad22380201d29be4c08c55df37d933123ba4ca0a03655f7b8779b02a" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/pin/tftp/v3",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/user",
    "io",
    "log",
    "strconv",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Handler", "struct", 22),
    ("tftpDriver", "struct", 31),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewTFTPHandler", "", 39),
    ("Start", "h *Handler", 65),
    ("Close", "h *Handler", 85),
    ("readHandler", "d *tftpDriver", 93),
    ("writeHandler", "d *tftpDriver", 148),
];

pub async fn newtftphandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/tftp.go", function: "NewTFTPHandler" })
}

pub async fn handler_start(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/tftp.go", function: "Handler.Start" })
}

pub async fn handler_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/tftp.go", function: "Handler.Close" })
}

pub async fn tftpdriver_readhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/tftp.go", function: "tftpDriver.readHandler" })
}

pub async fn tftpdriver_writehandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/tftp.go", function: "tftpDriver.writeHandler" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
