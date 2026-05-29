//! Original Go file: `notification.go`
//! Package: `main`; LOC: 67; SHA256: `e523b6f6c0c604f249160a8937e949070b3779bd047f16b10055f64ee8a5b700`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "notification.go", package: "main", go_loc: 67, functions: 1, types: 0, sha256: "e523b6f6c0c604f249160a8937e949070b3779bd047f16b10055f64ee8a5b700" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/notification",
    "imuslab.com/arozos/mod/notification/agents/smtpn",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("notificationInit", "", 14),
];

pub async fn notificationinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "notification.go", function: "notificationInit" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
