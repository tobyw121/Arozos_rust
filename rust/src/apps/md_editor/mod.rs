//! Markdown Editor Application
use axum::{Router, routing::get, response::Html, Json};
use serde::Serialize;

pub const APP_NAME: &str = "Markdown Editor";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Markdown editor with live preview";

#[derive(Debug, Serialize)]
pub struct MDEditorInfo {
    name: String,
    version: String,
    description: String,
}

pub async fn index() -> Html<String> {
    let html = r#"<!DOCTYPE html><html><head><title>Markdown Editor - arozOS</title><meta charset="utf-8"><style>body{margin:0;display:flex;height:100vh;font-family:Arial}.editor-container{display:flex;width:100%}#editor,#preview{flex:1;padding:20px;border:none;outline:none}#editor{background:#f5f5f5;resize:none;font-family:monospace}#preview{background:white;overflow-y:auto}button{padding:8px 16px;margin:4px;background:#007bff;color:white;border:none;border-radius:4px;cursor:pointer}</style></head><body><div style="display:flex;flex-direction:column;width:100%"><div style="padding:8px;background:#333"><button onclick="document.execCommand('bold')">Bold</button><button onclick="insertLink()">Link</button><button onclick="insertImage()">Image</button></div><div class="editor-container"><textarea id="editor" placeholder="Write Markdown here..."># Welcome to Markdown Editor

Start typing your **markdown** here!</textarea><div id="preview"></div></div></div><script>const editor=document.getElementById('editor');const preview=document.getElementById('preview');function render(){preview.innerHTML=marked.parse(editor.value)||'<p style=\"color:#999\">Preview will appear here...</p>'}editor.addEventListener('input',render);function insertLink(){const url=prompt('Enter URL:');if(url)editor.value+='\n['+url+']('+url+')'}function insertImage(){const url=prompt('Enter image URL:');if(url)editor.value+='\n![]('+url+')'}if(typeof marked==='undefined'){const s=document.createElement('script');s.src='https://cdn.jsdelivr.net/npm/marked/marked.min.js';s.onload=render;document.head.appendChild(s)}else{render()}</script></body></html>"#;
    Html(html.to_string())
}

pub async fn api_info() -> Json<MDEditorInfo> {
    Json(MDEditorInfo {
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
