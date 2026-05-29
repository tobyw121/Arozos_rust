//! Original Go file: `mod/fileservers/fileservers.go`
//! Package: `fileservers`; LOC: 18; SHA256: `20607b378c80b9be4bbdd20fd1991008b7c3104498b8aee4260525f6196a29bb`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/fileservers.go", package: "fileservers", go_loc: 18, functions: 1, types: 0, sha256: "20607b378c80b9be4bbdd20fd1991008b7c3104498b8aee4260525f6196a29bb" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetFileServerById", "", 10),
];

pub async fn getfileserverbyid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/fileservers.go", function: "GetFileServerById" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
