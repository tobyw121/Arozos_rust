//! Original Go file: `mod/storage/ftp/drivers.go`
//! Package: `ftp`; LOC: 100; SHA256: `19a332527682a934c8ffb8a6a5900e1bcb0bfa302ba1f5f18a24c34b61de8b4c`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/ftp/drivers.go", package: "ftp", go_loc: 100, functions: 5, types: 0, sha256: "19a332527682a934c8ffb8a6a5900e1bcb0bfa302ba1f5f18a24c34b61de8b4c" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/tls",
    "errors",
    "github.com/fclairamb/ftpserverlib",
    "log",
    "os",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetSettings", "m mainDriver", 13),
    ("ClientConnected", "m mainDriver", 17),
    ("ClientDisconnected", "m mainDriver", 23),
    ("AuthUser", "m mainDriver", 48),
    ("GetTLSConfig", "m mainDriver", 98),
];

pub async fn maindriver_getsettings(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/drivers.go", function: "mainDriver.GetSettings" })
}

pub async fn maindriver_clientconnected(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/drivers.go", function: "mainDriver.ClientConnected" })
}

pub async fn maindriver_clientdisconnected(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/drivers.go", function: "mainDriver.ClientDisconnected" })
}

pub async fn maindriver_authuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/drivers.go", function: "mainDriver.AuthUser" })
}

pub async fn maindriver_gettlsconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/drivers.go", function: "mainDriver.GetTLSConfig" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
