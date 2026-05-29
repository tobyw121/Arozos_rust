//! Original Go file: `mod/storage/sftpserver/sftpserver.go`
//! Package: `sftpserver`; LOC: 190; SHA256: `1146647b22027df963d1af3ad5421e5c0a3d4e3e0ed54ee6545743517fa9b5be`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/sftpserver/sftpserver.go", package: "sftpserver", go_loc: 190, functions: 2, types: 2, sha256: "1146647b22027df963d1af3ad5421e5c0a3d4e3e0ed54ee6545743517fa9b5be" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/pkg/sftp",
    "github.com/satori/go.uuid",
    "golang.org/x/crypto/ssh",
    "imuslab.com/arozos/mod/user",
    "io",
    "log",
    "net",
    "os",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SFTPConfig", "struct", 17),
    ("Instance", "struct", 23),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSFTPServer", "", 31),
    ("Close", "i *Instance", 186),
];

pub async fn newsftpserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/sftpserver.go", function: "NewSFTPServer" })
}

pub async fn instance_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/sftpserver.go", function: "Instance.Close" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
