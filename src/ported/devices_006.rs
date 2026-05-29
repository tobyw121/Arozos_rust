//! Original Go file: `devices.go`
//! Package: `main`; LOC: 49; SHA256: `595ab8cc972f99963278b9d288b221a2caa1505045386d4bac92017ce0664601`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "devices.go", package: "main", go_loc: 49, functions: 1, types: 0, sha256: "595ab8cc972f99963278b9d288b221a2caa1505045386d4bac92017ce0664601" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("DeviceServiceInit", "", 10),
];

pub async fn deviceserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "devices.go", function: "DeviceServiceInit" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
