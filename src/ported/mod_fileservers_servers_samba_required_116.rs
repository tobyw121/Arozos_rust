//! Original Go file: `mod/fileservers/servers/samba/required.go`
//! Package: `samba`; LOC: 83; SHA256: `176149d8d609ed545b1847c3579a35c203ca90d54d1e0b8357a0349edf53f819`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/samba/required.go", package: "samba", go_loc: 83, functions: 4, types: 0, sha256: "176149d8d609ed545b1847c3579a35c203ca90d54d1e0b8357a0349edf53f819" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/user",
    "log",
    "os/exec",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ServerToggle", "m *ShareManager", 17),
    ("IsEnabled", "m *ShareManager", 21),
    ("GetEndpoints", "m *ShareManager", 31),
    ("checkSmbdRunning", "", 63),
];

pub async fn sharemanager_servertoggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/required.go", function: "ShareManager.ServerToggle" })
}

pub async fn sharemanager_isenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/required.go", function: "ShareManager.IsEnabled" })
}

pub async fn sharemanager_getendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/required.go", function: "ShareManager.GetEndpoints" })
}

pub async fn checksmbdrunning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/required.go", function: "checkSmbdRunning" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
