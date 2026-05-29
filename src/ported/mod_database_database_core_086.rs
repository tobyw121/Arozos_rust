//! Original Go file: `mod/database/database_core.go`
//! Package: `database`; LOC: 189; SHA256: `4acc04e18d40f4fac06365fdac0464f07bfa73e01dc2f7dee412f37602261dd0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/database/database_core.go", package: "database", go_loc: 189, functions: 11, types: 0, sha256: "4acc04e18d40f4fac06365fdac0464f07bfa73e01dc2f7dee412f37602261dd0" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/boltdb/bolt",
    "log",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("newDatabase", "", 15),
    ("dump", "d *Database", 38),
    ("newTable", "d *Database", 57),
    ("tableExists", "d *Database", 75),
    ("dropTable", "d *Database", 83),
    ("write", "d *Database", 102),
    ("read", "d *Database", 123),
    ("keyExists", "d *Database", 133),
    ("delete", "d *Database", 160),
    ("listTable", "d *Database", 173),
    ("close", "d *Database", 187),
];

pub async fn newdatabase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "newDatabase" })
}

pub async fn database_dump(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.dump" })
}

pub async fn database_newtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.newTable" })
}

pub async fn database_tableexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.tableExists" })
}

pub async fn database_droptable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.dropTable" })
}

pub async fn database_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.write" })
}

pub async fn database_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.read" })
}

pub async fn database_keyexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.keyExists" })
}

pub async fn database_delete(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.delete" })
}

pub async fn database_listtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.listTable" })
}

pub async fn database_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database_core.go", function: "Database.close" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
