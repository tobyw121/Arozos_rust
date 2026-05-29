//! Original Go file: `mod/time/scheduler/scheduler.go`
//! Package: `scheduler`; LOC: 230; SHA256: `72cdbdf2eb26197ab581623ea5ca855fd58757a9f7fdf39f441e20ad3eb31149`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/time/scheduler/scheduler.go", package: "scheduler", go_loc: 230, functions: 8, types: 3, sha256: "72cdbdf2eb26197ab581623ea5ca855fd58757a9f7fdf39f441e20ad3eb31149" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/agi",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "path/filepath",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Job", "struct", 25),
    ("ScheudlerOption", "struct", 38),
    ("Scheduler", "struct", 45),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewScheduler", "", 53),
    ("createTicker", "a *Scheduler", 93),
    ("Close", "a *Scheduler", 167),
    ("AddJobToScheduler", "a *Scheduler", 175),
    ("GetScheduledJobByName", "a *Scheduler", 180),
    ("RemoveJobFromScheduleList", "a *Scheduler", 190),
    ("JobExists", "a *Scheduler", 201),
    ("cronlog", "", 212),
];

pub async fn newscheduler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "NewScheduler" })
}

pub async fn scheduler_createticker(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "Scheduler.createTicker" })
}

pub async fn scheduler_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "Scheduler.Close" })
}

pub async fn scheduler_addjobtoscheduler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "Scheduler.AddJobToScheduler" })
}

pub async fn scheduler_getscheduledjobbyname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "Scheduler.GetScheduledJobByName" })
}

pub async fn scheduler_removejobfromschedulelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "Scheduler.RemoveJobFromScheduleList" })
}

pub async fn scheduler_jobexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "Scheduler.JobExists" })
}

pub async fn cronlog(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/time/scheduler/scheduler.go", function: "cronlog" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
