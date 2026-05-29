//! Original Go file: `mod/quota/quota.go`
//! Package: `quota`; LOC: 158; SHA256: `f11fc768601f94521bbba557f546c102783379c14554d0558ed1f75e661fdb49`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/quota/quota.go", package: "quota", go_loc: 158, functions: 11, types: 1, sha256: "f11fc768601f94521bbba557f546c102783379c14554d0558ed1f75e661fdb49" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/filesystem",
    "log",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("QuotaHandler", "struct", 21),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewUserQuotaHandler", "", 30),
    ("SetUserStorageQuota", "q *QuotaHandler", 65),
    ("GetUserStorageQuota", "q *QuotaHandler", 70),
    ("IsQuotaInitialized", "q *QuotaHandler", 80),
    ("RemoveUserQuota", "q *QuotaHandler", 88),
    ("HaveSpace", "q *QuotaHandler", 92),
    ("UpdateUserStoragePool", "q *QuotaHandler", 105),
    ("AllocateSpace", "q *QuotaHandler", 110),
    ("ReclaimSpace", "q *QuotaHandler", 116),
    ("CalculateQuotaUsage", "q *QuotaHandler", 125),
    ("inSlice", "", 151),
];

pub async fn newuserquotahandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "NewUserQuotaHandler" })
}

pub async fn quotahandler_setuserstoragequota(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.SetUserStorageQuota" })
}

pub async fn quotahandler_getuserstoragequota(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.GetUserStorageQuota" })
}

pub async fn quotahandler_isquotainitialized(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.IsQuotaInitialized" })
}

pub async fn quotahandler_removeuserquota(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.RemoveUserQuota" })
}

pub async fn quotahandler_havespace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.HaveSpace" })
}

pub async fn quotahandler_updateuserstoragepool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.UpdateUserStoragePool" })
}

pub async fn quotahandler_allocatespace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.AllocateSpace" })
}

pub async fn quotahandler_reclaimspace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.ReclaimSpace" })
}

pub async fn quotahandler_calculatequotausage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "QuotaHandler.CalculateQuotaUsage" })
}

pub async fn inslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/quota/quota.go", function: "inSlice" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
