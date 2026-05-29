//! Original Go file: `mod/database/database.go`
//! Package: `database`; LOC: 120; SHA256: `0d2a2a6050ba72f2c10b7a16adfb65e84e94319c3663b48cd99b0d9931488aa3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/database/database.go", package: "database", go_loc: 120, functions: 12, types: 1, sha256: "0d2a2a6050ba72f2c10b7a16adfb65e84e94319c3663b48cd99b0d9931488aa3" };

pub const GO_IMPORTS: &[&str] = &[
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Database", "struct", 15),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDatabase", "", 21),
    ("UpdateReadWriteMode", "d *Database", 32),
    ("Dump", "d *Database", 37),
    ("NewTable", "d *Database", 42),
    ("TableExists", "d *Database", 47),
    ("DropTable", "d *Database", 52),
    ("Write", "d *Database", 66),
    ("Read", "d *Database", 80),
    ("KeyExists", "d *Database", 84),
    ("Delete", "d *Database", 93),
    ("ListTable", "d *Database", 114),
    ("Close", "d *Database", 118),
];

pub async fn newdatabase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "NewDatabase" })
}

pub async fn database_updatereadwritemode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.UpdateReadWriteMode" })
}

pub async fn database_dump(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.Dump" })
}

pub async fn database_newtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.NewTable" })
}

pub async fn database_tableexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.TableExists" })
}

pub async fn database_droptable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.DropTable" })
}

pub async fn database_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.Write" })
}

pub async fn database_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.Read" })
}

pub async fn database_keyexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.KeyExists" })
}

pub async fn database_delete(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.Delete" })
}

pub async fn database_listtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.ListTable" })
}

pub async fn database_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/database/database.go", function: "Database.Close" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
