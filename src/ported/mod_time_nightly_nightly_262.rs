//! Original Go file: `mod/time/nightly/nightly.go`
//! Package: `nightly`; LOC: 50; SHA256: `cece842b1f9a07c2b8059f86f8f219d946ad4a722b2f59022142c0819635bba7`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/time/nightly/nightly.go", package: "nightly", go_loc: 50, functions: 3, types: 1, sha256: "cece842b1f9a07c2b8059f86f8f219d946ad4a722b2f59022142c0819635bba7" };

pub const GO_IMPORTS: &[&str] = &[
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("TaskManager", "struct", 14),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewNightlyTaskManager", "", 18),
    ("NightlyTaskRun", "tm *TaskManager", 42),
    ("RegisterNightlyTask", "tm *TaskManager", 48),
];

pub async fn newnightlytaskmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/nightly/nightly.go", function: "NewNightlyTaskManager" })
}

pub async fn taskmanager_nightlytaskrun(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/nightly/nightly.go", function: "TaskManager.NightlyTaskRun" })
}

pub async fn taskmanager_registernightlytask(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/nightly/nightly.go", function: "TaskManager.RegisterNightlyTask" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
