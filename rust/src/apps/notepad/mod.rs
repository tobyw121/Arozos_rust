//! Notepad Application
use axum::{Router, routing::get, response::Html, Json};
use serde::Serialize;

pub const APP_NAME: &str = "Notepad";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Simple text editor";

#[derive(Debug, Serialize)]
pub struct NotepadInfo {
    name: String,
    version: String,
    description: String,
}

pub async fn index() -> Html<String> {
    let html = r#"<!DOCTYPE html><html><head><title>Notepad - arozOS</title><meta charset="utf-8"><style>body{margin:0;display:flex;flex-direction:column;height:100vh;font-family:Arial}textarea{flex:1;padding:20px;border:none;outline:none;font-family:monospace;font-size:14px;resize:none}.toolbar{padding:8px;background:#f0f0f0;display:flex;gap:8px}button{padding:6px 12px;background:#007bff;color:white;border:none;border-radius:4px;cursor:pointer}</style></head><body><div class="toolbar"><button onclick="newFile()">New</button><button onclick="saveFile()">Save</button><button onclick="document.execCommand('undo')">Undo</button></div><textarea id="editor" placeholder="Start typing..."></textarea><script>function newFile(){document.getElementById('editor').value=''}function saveFile(){const blob=new Blob([document.getElementById('editor').value],{type:'text/plain'});const a=document.createElement('a');a.href=URL.createObjectURL(blob);a.download='untitled.txt';a.click()}</script></body></html>"#;
    Html(html.to_string())
}

pub async fn api_info() -> Json<NotepadInfo> {
    Json(NotepadInfo {
        name: APP_NAME.to_string(),
        version: APP_VERSION.to_string(),
        description: APP_DESCRIPTION.to_string(),
    })
}

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
}
