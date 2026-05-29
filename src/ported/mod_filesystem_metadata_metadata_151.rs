//! Original Go file: `mod/filesystem/metadata/metadata.go`
//! Package: `metadata`; LOC: 411; SHA256: `5f0703c95d02a389c6579e356e251f191d44d36a449dc765a9a799fe97b8848a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/metadata.go", package: "metadata", go_loc: 411, functions: 13, types: 1, sha256: "5f0703c95d02a389c6579e356e251f191d44d36a449dc765a9a799fe97b8848a" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/base64",
    "encoding/json",
    "errors",
    "github.com/gorilla/websocket",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/fssort",
    "imuslab.com/arozos/mod/filesystem/hidden",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("RenderHandler", "struct", 38),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("IsRawImageFile", "", 33),
    ("NewRenderHandler", "", 44),
    ("BuildCacheForFolder", "rh *RenderHandler", 52),
    ("LoadCacheAsBytes", "rh *RenderHandler", 74),
    ("LoadCache", "rh *RenderHandler", 87),
    ("checkCacheNeeded", "rh *RenderHandler", 108),
    ("generateCache", "rh *RenderHandler", 150),
    ("fileIsBusy", "rh *RenderHandler", 220),
    ("getImageAsBase64", "", 234),
    ("HandleLoadCache", "rh *RenderHandler", 245),
    ("CacheExists", "", 373),
    ("GetCacheFilePath", "", 379),
    ("RemoveCache", "", 396),
];

pub async fn israwimagefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "IsRawImageFile" })
}

pub async fn newrenderhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "NewRenderHandler" })
}

pub async fn renderhandler_buildcacheforfolder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.BuildCacheForFolder" })
}

pub async fn renderhandler_loadcacheasbytes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.LoadCacheAsBytes" })
}

pub async fn renderhandler_loadcache(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.LoadCache" })
}

pub async fn renderhandler_checkcacheneeded(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.checkCacheNeeded" })
}

pub async fn renderhandler_generatecache(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.generateCache" })
}

pub async fn renderhandler_fileisbusy(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.fileIsBusy" })
}

pub async fn getimageasbase64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "getImageAsBase64" })
}

pub async fn renderhandler_handleloadcache(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RenderHandler.HandleLoadCache" })
}

pub async fn cacheexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "CacheExists" })
}

pub async fn getcachefilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "GetCacheFilePath" })
}

pub async fn removecache(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/metadata.go", function: "RemoveCache" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
