//! Original Go file: `file_system.go`
//! Package: `main`; LOC: 3448; SHA256: `f912fdb6cec3b354724727c01c126157402d492cbced989e64749107006658df`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "file_system.go", package: "main", go_loc: 3448, functions: 40, types: 2, sha256: "f912fdb6cec3b354724727c01c126157402d492cbced989e64749107006658df" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/sha256",
    "encoding/hex",
    "encoding/json",
    "errors",
    "github.com/gorilla/websocket",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/compatibility",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/fspermission",
    "imuslab.com/arozos/mod/filesystem/fssort",
    "imuslab.com/arozos/mod/filesystem/fuzzy",
    "imuslab.com/arozos/mod/filesystem/hidden",
    "imuslab.com/arozos/mod/filesystem/localversion",
    "imuslab.com/arozos/mod/filesystem/metadata",
    "imuslab.com/arozos/mod/filesystem/shortcut",
    "imuslab.com/arozos/mod/modules",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/share",
    "imuslab.com/arozos/mod/share/shareEntry",
    "imuslab.com/arozos/mod/storage",
    "imuslab.com/arozos/mod/utils",
    "io",
    "io/fs",
    "log",
    "math",
    "mime",
    "net/http",
    "net/url",
    "os",
    "path/filepath",
    "runtime",
    "sort",
    "strconv",
    "strings",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("trashedFile", "struct", 53),
    ("fileOperationTask", "struct", 65),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FileSystemInit", "", 75),
    ("system_fs_handleFileSearch", "", 252),
    ("system_fs_handleLowMemoryUpload", "", 383),
    ("system_fs_handleUpload", "", 746),
    ("system_fs_validateFileOpr", "", 888),
    ("system_fs_WebSocketScanTrashBin", "", 930),
    ("system_fs_scanTrashBin", "", 1021),
    ("system_fs_restoreFile", "", 1074),
    ("system_fs_clearTrashBin", "", 1124),
    ("system_fs_listTrash", "", 1159),
    ("system_fs_handleNewObjects", "", 1203),
    ("system_fs_handleWebSocketOpr", "", 1340),
    ("system_fs_handleOpr", "", 1777),
    ("system_fs_handleUserPreference", "", 2217),
    ("system_fs_removeUserPreferences", "", 2257),
    ("system_fs_listDrives", "", 2271),
    ("system_fs_listRoot", "", 2324),
    ("system_fs_specialURIDecode", "", 2385),
    ("system_fs_specialURIEncode", "", 2393),
    ("system_fs_getFileProperties", "", 2402),
    ("system_fs_handleList", "", 2537),
    ("system_fs_handleDirHash", "", 2688),
    ("system_fs_zipHandler", "", 2761),
    ("system_fs_FileVersionHistory", "", 2878),
    ("system_fs_clearVersionHistories", "", 2974),
    ("system_fs_handleCacheRender", "", 2985),
    ("system_fs_handleThumbnailLoad", "", 3012),
    ("system_fs_handleFolderCache", "", 3063),
    ("system_fs_handleFolderSortModePreference", "", 3082),
    ("system_fs_handleFilePermission", "", 3131),
    ("system_fs_clearOldTmpFiles", "", 3223),
    ("getFsBufferFilepath", "", 3284),
    ("bufferRemoteFileToLocal", "", 3297),
    ("isFsBufferFilepath", "", 3318),
    ("cleanFsBufferFileFromList", "", 3327),
    ("system_fs_HandleOnGoingTasks", "", 3346),
    ("GetAllOngoingFileOperationForUser", "", 3407),
    ("GetOngoingFileOperationByOprID", "", 3422),
    ("SetOngoingFileOperation", "", 3432),
    ("UpdateOngoingFileOperation", "", 3437),
];

pub async fn filesysteminit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "FileSystemInit" })
}

pub async fn system_fs_handlefilesearch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleFileSearch" })
}

pub async fn system_fs_handlelowmemoryupload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleLowMemoryUpload" })
}

pub async fn system_fs_handleupload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleUpload" })
}

pub async fn system_fs_validatefileopr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_validateFileOpr" })
}

pub async fn system_fs_websocketscantrashbin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_WebSocketScanTrashBin" })
}

pub async fn system_fs_scantrashbin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_scanTrashBin" })
}

pub async fn system_fs_restorefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_restoreFile" })
}

pub async fn system_fs_cleartrashbin(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_clearTrashBin" })
}

pub async fn system_fs_listtrash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_listTrash" })
}

pub async fn system_fs_handlenewobjects(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleNewObjects" })
}

pub async fn system_fs_handlewebsocketopr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleWebSocketOpr" })
}

pub async fn system_fs_handleopr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleOpr" })
}

pub async fn system_fs_handleuserpreference(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleUserPreference" })
}

pub async fn system_fs_removeuserpreferences(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_removeUserPreferences" })
}

pub async fn system_fs_listdrives(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_listDrives" })
}

pub async fn system_fs_listroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_listRoot" })
}

pub async fn system_fs_specialuridecode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_specialURIDecode" })
}

pub async fn system_fs_specialuriencode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_specialURIEncode" })
}

pub async fn system_fs_getfileproperties(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_getFileProperties" })
}

pub async fn system_fs_handlelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleList" })
}

pub async fn system_fs_handledirhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleDirHash" })
}

pub async fn system_fs_ziphandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_zipHandler" })
}

pub async fn system_fs_fileversionhistory(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_FileVersionHistory" })
}

pub async fn system_fs_clearversionhistories(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_clearVersionHistories" })
}

pub async fn system_fs_handlecacherender(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleCacheRender" })
}

pub async fn system_fs_handlethumbnailload(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleThumbnailLoad" })
}

pub async fn system_fs_handlefoldercache(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleFolderCache" })
}

pub async fn system_fs_handlefoldersortmodepreference(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleFolderSortModePreference" })
}

pub async fn system_fs_handlefilepermission(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_handleFilePermission" })
}

pub async fn system_fs_clearoldtmpfiles(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_clearOldTmpFiles" })
}

pub async fn getfsbufferfilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "getFsBufferFilepath" })
}

pub async fn bufferremotefiletolocal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "bufferRemoteFileToLocal" })
}

pub async fn isfsbufferfilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "isFsBufferFilepath" })
}

pub async fn cleanfsbufferfilefromlist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "cleanFsBufferFileFromList" })
}

pub async fn system_fs_handleongoingtasks(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "system_fs_HandleOnGoingTasks" })
}

pub async fn getallongoingfileoperationforuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "GetAllOngoingFileOperationForUser" })
}

pub async fn getongoingfileoperationbyoprid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "GetOngoingFileOperationByOprID" })
}

pub async fn setongoingfileoperation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "SetOngoingFileOperation" })
}

pub async fn updateongoingfileoperation(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "file_system.go", function: "UpdateOngoingFileOperation" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
