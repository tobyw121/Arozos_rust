//! Original Go file: `mod/network/webdav/file_test.go`
//! Package: `webdav`; LOC: 1182; SHA256: `9b0dee41d5038dd798869ccf4e6759d0e9a8fd953cfa9492007c2d3d0923faaa`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/file_test.go", package: "webdav", go_loc: 1182, functions: 15, types: 0, sha256: "9b0dee41d5038dd798869ccf4e6759d0e9a8fd953cfa9492007c2d3d0923faaa" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "encoding/xml",
    "fmt",
    "io",
    "os",
    "path",
    "path/filepath",
    "reflect",
    "runtime",
    "sort",
    "strconv",
    "strings",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestSlashClean", "", 23),
    ("TestDirResolve", "", 47),
    ("TestWalk", "", 154),
    ("find", "", 236),
    ("testFS", "", 262),
    ("TestDir", "", 512),
    ("TestMemFS", "", 528),
    ("TestMemFSRoot", "", 532),
    ("TestMemFileReaddir", "", 567),
    ("TestMemFile", "", 589),
    ("TestMemFileWriteAllocs", "", 796),
    ("BenchmarkMemFileWrite", "", 823),
    ("TestCopyMoveProps", "", 849),
    ("TestWalkFS", "", 959),
    ("buildTestFS", "", 1148),
];

pub async fn testslashclean(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestSlashClean" })
}

pub async fn testdirresolve(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestDirResolve" })
}

pub async fn testwalk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestWalk" })
}

pub async fn find(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "find" })
}

pub async fn testfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "testFS" })
}

pub async fn testdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestDir" })
}

pub async fn testmemfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestMemFS" })
}

pub async fn testmemfsroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestMemFSRoot" })
}

pub async fn testmemfilereaddir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestMemFileReaddir" })
}

pub async fn testmemfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestMemFile" })
}

pub async fn testmemfilewriteallocs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestMemFileWriteAllocs" })
}

pub async fn benchmarkmemfilewrite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "BenchmarkMemFileWrite" })
}

pub async fn testcopymoveprops(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestCopyMoveProps" })
}

pub async fn testwalkfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "TestWalkFS" })
}

pub async fn buildtestfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/file_test.go", function: "buildTestFS" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
