//! Original Go file: `quota.go`
//! Package: `main`; LOC: 162; SHA256: `d38f4b6953f37b9ac239fff2761b52be6c29303e88ae3a3edd499a7d336d967a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "quota.go", package: "main", go_loc: 162, functions: 5, types: 0, sha256: "d38f4b6953f37b9ac239fff2761b52be6c29303e88ae3a3edd499a7d336d967a" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "os",
    "path/filepath",
    "sort",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("DiskQuotaInit", "", 17),
    ("system_disk_quota_updateAllUserQuotaEstimation", "", 37),
    ("system_disk_quota_setQuota", "", 47),
    ("system_disk_quota_handleQuotaInfo", "", 85),
    ("system_disk_quota_handleFileDistributionView", "", 114),
];

pub async fn diskquotainit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "quota.go", function: "DiskQuotaInit" })
}

pub async fn system_disk_quota_updatealluserquotaestimation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "quota.go", function: "system_disk_quota_updateAllUserQuotaEstimation" })
}

pub async fn system_disk_quota_setquota(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "quota.go", function: "system_disk_quota_setQuota" })
}

pub async fn system_disk_quota_handlequotainfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "quota.go", function: "system_disk_quota_handleQuotaInfo" })
}

pub async fn system_disk_quota_handlefiledistributionview(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "quota.go", function: "system_disk_quota_handleFileDistributionView" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
