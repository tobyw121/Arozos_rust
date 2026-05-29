//! Original Go file: `mod/auth/authlogger/authlogger_test.go`
//! Package: `authlogger`; LOC: 204; SHA256: `7431483ae99b2ac7e908a0987455cf6eda14906448bb524e56c7fa7e0e04bc6d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/authlogger/authlogger_test.go", package: "authlogger", go_loc: 204, functions: 6, types: 0, sha256: "7431483ae99b2ac7e908a0987455cf6eda14906448bb524e56c7fa7e0e04bc6d" };

pub const GO_IMPORTS: &[&str] = &[
    "net/http",
    "net/url",
    "os",
    "testing",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("setupSuite", "", 13),
    ("TestNewLogger", "", 29),
    ("TestLogAuth", "", 46),
    ("TestListSummary", "", 106),
    ("TestListRecords", "", 126),
    ("TestLogAuthByRequestInfo", "", 157),
];

pub async fn setupsuite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger_test.go", function: "setupSuite" })
}

pub async fn testnewlogger(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger_test.go", function: "TestNewLogger" })
}

pub async fn testlogauth(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger_test.go", function: "TestLogAuth" })
}

pub async fn testlistsummary(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger_test.go", function: "TestListSummary" })
}

pub async fn testlistrecords(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger_test.go", function: "TestListRecords" })
}

pub async fn testlogauthbyrequestinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/authlogger/authlogger_test.go", function: "TestLogAuthByRequestInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
