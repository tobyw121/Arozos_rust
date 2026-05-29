//! Original Go file: `mod/storage/bridge/bridge.go`
//! Package: `bridge`; LOC: 130; SHA256: `daa6320c37fe11e893111cc63d4dce863187881b72904cb5b546f9dfc42debc1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/bridge/bridge.go", package: "bridge", go_loc: 130, functions: 7, types: 2, sha256: "daa6320c37fe11e893111cc63d4dce863187881b72904cb5b546f9dfc42debc1" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "os",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Record", "struct", 17),
    ("BridgeConfig", "struct", 21),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewBridgeRecord", "", 26),
    ("ReadConfig", "r *Record", 33),
    ("AppendToConfig", "r *Record", 55),
    ("RemoveFromConfig", "r *Record", 76),
    ("IsBridgedFSH", "r *Record", 95),
    ("GetBridgedGroups", "r *Record", 110),
    ("WriteConfig", "r *Record", 126),
];

pub async fn newbridgerecord(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "NewBridgeRecord" })
}

pub async fn record_readconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "Record.ReadConfig" })
}

pub async fn record_appendtoconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "Record.AppendToConfig" })
}

pub async fn record_removefromconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "Record.RemoveFromConfig" })
}

pub async fn record_isbridgedfsh(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "Record.IsBridgedFSH" })
}

pub async fn record_getbridgedgroups(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "Record.GetBridgedGroups" })
}

pub async fn record_writeconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/bridge/bridge.go", function: "Record.WriteConfig" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
