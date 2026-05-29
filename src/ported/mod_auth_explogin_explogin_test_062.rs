//! Original Go file: `mod/auth/explogin/explogin_test.go`
//! Package: `explogin`; LOC: 142; SHA256: `a49a3f07f06517762c933c7df182f25b2067bc4fd07aa7ddaed8436b1e637717`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/explogin/explogin_test.go", package: "explogin", go_loc: 142, functions: 8, types: 0, sha256: "a49a3f07f06517762c933c7df182f25b2067bc4fd07aa7ddaed8436b1e637717" };

pub const GO_IMPORTS: &[&str] = &[
    "net/http",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestAllowImmediateAccess_FirstAttempt", "", 8),
    ("TestAllowImmediateAccess_LimitExceeded", "", 20),
    ("TestAddUserRetrycount", "", 37),
    ("TestResetUserRetryCount", "", 57),
    ("TestResetAllUserRetryCounter", "", 72),
    ("TestGetDelayTimeFromRetryCount", "", 91),
    ("TestAllowImmediateAccess_DeniedUntilNextRetry", "", 116),
    ("TestAllowImmediateAccess_IPNotFound", "", 132),
];

pub async fn testallowimmediateaccess_firstattempt(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestAllowImmediateAccess_FirstAttempt" })
}

pub async fn testallowimmediateaccess_limitexceeded(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestAllowImmediateAccess_LimitExceeded" })
}

pub async fn testadduserretrycount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestAddUserRetrycount" })
}

pub async fn testresetuserretrycount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestResetUserRetryCount" })
}

pub async fn testresetalluserretrycounter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestResetAllUserRetryCounter" })
}

pub async fn testgetdelaytimefromretrycount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestGetDelayTimeFromRetryCount" })
}

pub async fn testallowimmediateaccess_denieduntilnextretry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestAllowImmediateAccess_DeniedUntilNextRetry" })
}

pub async fn testallowimmediateaccess_ipnotfound(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/explogin/explogin_test.go", function: "TestAllowImmediateAccess_IPNotFound" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
