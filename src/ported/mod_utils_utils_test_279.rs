//! Original Go file: `mod/utils/utils_test.go`
//! Package: `utils`; LOC: 96; SHA256: `31571b49d146f9af7d7105d854a6caa4fe53cd7d498493fd9b511d6c34157e1c`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/utils/utils_test.go", package: "utils", go_loc: 96, functions: 6, types: 0, sha256: "31571b49d146f9af7d7105d854a6caa4fe53cd7d498493fd9b511d6c34157e1c" };

pub const GO_IMPORTS: &[&str] = &[
    "net/http/httptest",
    "os",
    "testing",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestSendTextResponse", "", 10),
    ("TestSendJSONResponse", "", 19),
    ("TestSendErrorResponse", "", 33),
    ("TestSendOK", "", 47),
    ("TestTimeToString", "", 61),
    ("TestFileExists", "", 71),
];

pub async fn testsendtextresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils_test.go", function: "TestSendTextResponse" })
}

pub async fn testsendjsonresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils_test.go", function: "TestSendJSONResponse" })
}

pub async fn testsenderrorresponse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils_test.go", function: "TestSendErrorResponse" })
}

pub async fn testsendok(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils_test.go", function: "TestSendOK" })
}

pub async fn testtimetostring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils_test.go", function: "TestTimeToString" })
}

pub async fn testfileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/utils/utils_test.go", function: "TestFileExists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
