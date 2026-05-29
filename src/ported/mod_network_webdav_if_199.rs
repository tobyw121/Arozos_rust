//! Original Go file: `mod/network/webdav/if.go`
//! Package: `webdav`; LOC: 173; SHA256: `d8ea0d54abda91a043ea2bb372710d6d92930062b33c0c869c923bdd51f5ec50`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/if.go", package: "webdav", go_loc: 173, functions: 6, types: 2, sha256: "d8ea0d54abda91a043ea2bb372710d6d92930062b33c0c869c923bdd51f5ec50" };

pub const GO_IMPORTS: &[&str] = &[
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ifHeader", "struct", 15),
    ("ifList", "struct", 20),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("parseIfHeader", "", 28),
    ("parseNoTagLists", "", 40),
    ("parseTaggedLists", "", 54),
    ("parseList", "", 83),
    ("parseCondition", "", 105),
    ("lex", "", 133),
];

pub async fn parseifheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/if.go", function: "parseIfHeader" })
}

pub async fn parsenotaglists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/if.go", function: "parseNoTagLists" })
}

pub async fn parsetaggedlists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/if.go", function: "parseTaggedLists" })
}

pub async fn parselist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/if.go", function: "parseList" })
}

pub async fn parsecondition(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/if.go", function: "parseCondition" })
}

pub async fn lex(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/if.go", function: "lex" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
