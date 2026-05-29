//! Original Go file: `mod/database/database_openwrt.go`
//! Package: `database`; LOC: 208; SHA256: `1988ef5419c581d0ced0b0578f9cb4dcbbe0ff905586e6201efe23cf8d313fb5`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/database/database_openwrt.go", package: "database", go_loc: 208, functions: 13, types: 0, sha256: "1988ef5419c581d0ced0b0578f9cb4dcbbe0ff905586e6201efe23cf8d313fb5" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "log",
    "os",
    "path/filepath",
    "strings",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("newDatabase", "", 16),
    ("dump", "d *Database", 45),
    ("newTable", "d *Database", 63),
    ("tableExists", "d *Database", 74),
    ("dropTable", "d *Database", 87),
    ("write", "d *Database", 100),
    ("read", "d *Database", 115),
    ("keyExists", "d *Database", 133),
    ("delete", "d *Database", 140),
    ("listTable", "d *Database", 154),
    ("close", "d *Database", 186),
    ("isDirectory", "", 190),
    ("fileExists", "", 199),
];

pub async fn newdatabase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "newDatabase" })
}

pub async fn database_dump(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.dump" })
}

pub async fn database_newtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.newTable" })
}

pub async fn database_tableexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.tableExists" })
}

pub async fn database_droptable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.dropTable" })
}

pub async fn database_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.write" })
}

pub async fn database_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.read" })
}

pub async fn database_keyexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.keyExists" })
}

pub async fn database_delete(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.delete" })
}

pub async fn database_listtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.listTable" })
}

pub async fn database_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "Database.close" })
}

pub async fn isdirectory(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "isDirectory" })
}

pub async fn fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_openwrt.go", function: "fileExists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
