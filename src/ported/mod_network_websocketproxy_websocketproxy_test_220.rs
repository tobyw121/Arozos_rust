//! Original Go file: `mod/network/websocketproxy/websocketproxy_test.go`
//! Package: `websocketproxy`; LOC: 131; SHA256: `900fc5ae608c43d9d90317a8cff95da1f057f114309199a976012dcaa59c2b99`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/websocketproxy/websocketproxy_test.go", package: "websocketproxy", go_loc: 131, functions: 1, types: 0, sha256: "900fc5ae608c43d9d90317a8cff95da1f057f114309199a976012dcaa59c2b99" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/gorilla/websocket",
    "log",
    "net/http",
    "net/url",
    "testing",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestProxy", "", 18),
];

pub async fn testproxy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/websocketproxy/websocketproxy_test.go", function: "TestProxy" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
