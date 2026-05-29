//! Original Go file: `mod/auth/oauth2/syncdb/syncdb_test.go`
//! Package: `syncdb`; LOC: 95; SHA256: `4216718bdc40a3d5d378e31858adbdce7d29e4a098ddee35567ac3a434bd957b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/syncdb/syncdb_test.go", package: "syncdb", go_loc: 95, functions: 5, types: 0, sha256: "4216718bdc40a3d5d378e31858adbdce7d29e4a098ddee35567ac3a434bd957b" };

pub const GO_IMPORTS: &[&str] = &[
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestSyncDB_Store", "", 7),
    ("TestSyncDB_Read", "", 22),
    ("TestSyncDB_Delete", "", 45),
    ("TestSyncDB_AutoCleaning", "", 64),
    ("TestSyncDB_ToString", "", 83),
];

pub async fn testsyncdb_store(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/syncdb/syncdb_test.go", function: "TestSyncDB_Store" })
}

pub async fn testsyncdb_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/syncdb/syncdb_test.go", function: "TestSyncDB_Read" })
}

pub async fn testsyncdb_delete(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/syncdb/syncdb_test.go", function: "TestSyncDB_Delete" })
}

pub async fn testsyncdb_autocleaning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/syncdb/syncdb_test.go", function: "TestSyncDB_AutoCleaning" })
}

pub async fn testsyncdb_tostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/syncdb/syncdb_test.go", function: "TestSyncDB_ToString" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
