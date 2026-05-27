//! Code Studio Application
//! 
//! Integrated development environment for coding within arozOS.

use axum::{Router, routing::get, response::Html, Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub const APP_NAME: &str = "Code Studio";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Web-based code editor and IDE";

#[derive(Debug, Serialize)]
pub struct CodeStudioInfo {
    name: String,
    version: String,
    description: String,
    supported_languages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveFileRequest {
    path: String,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct SaveFileResponse {
    success: bool,
    message: String,
}

/// Main code editor interface
pub async fn index() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Code Studio - arozOS</title>
    <meta charset="utf-8">
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Consolas', 'Monaco', monospace; background: #1e1e1e; color: #d4d4d4; height: 100vh; overflow: hidden; }
        .ide-container { display: flex; flex-direction: column; height: 100vh; }
        .toolbar { display: flex; align-items: center; padding: 8px; background: #2d2d30; border-bottom: 1px solid #3e3e42; }
        .toolbar button { padding: 6px 12px; margin-right: 8px; background: #0e639c; color: white; border: none; border-radius: 4px; cursor: pointer; }
        .toolbar button:hover { background: #1177bb; }
        .file-path { flex: 1; padding: 6px; background: #3c3c3c; border: 1px solid #555; color: #ccc; border-radius: 4px; }
        .main-area { display: flex; flex: 1; overflow: hidden; }
        .sidebar { width: 200px; background: #252526; border-right: 1px solid #3e3e42; overflow-y: auto; }
        .sidebar-item { padding: 8px 12px; cursor: pointer; }
        .sidebar-item:hover { background: #2a2d2e; }
        .editor-container { flex: 1; position: relative; }
        #editor { width: 100%; height: 100%; border: none; outline: none; padding: 20px; background: #1e1e1e; color: #d4d4d4; font-size: 14px; line-height: 1.6; resize: none; }
        .status-bar { padding: 4px 12px; background: #007acc; color: white; font-size: 12px; }
    </style>
</head>
<body>
    <div class="ide-container">
        <div class="toolbar">
            <button onclick="newFile()">📄 New</button>
            <button onclick="openFile()">📂 Open</button>
            <button onclick="saveFile()">💾 Save</button>
            <input type="text" class="file-path" id="filePath" placeholder="File path..." value="/home/user/main.rs">
            <select id="language" style="padding: 6px; background: #3c3c3c; color: #ccc; border: 1px solid #555; border-radius: 4px;">
                <option value="rust">Rust</option>
                <option value="javascript">JavaScript</option>
                <option value="python">Python</option>
                <option value="go">Go</option>
                <option value="html">HTML</option>
                <option value="css">CSS</option>
            </select>
        </div>
        <div class="main-area">
            <div class="sidebar">
                <div class="sidebar-item">📁 src/</div>
                <div class="sidebar-item">📁 tests/</div>
                <div class="sidebar-item">📄 Cargo.toml</div>
                <div class="sidebar-item">📄 README.md</div>
            </div>
            <div class="editor-container">
                <textarea id="editor" spellcheck="false">// Welcome to Code Studio!
// Start coding here...

fn main() {
    println!("Hello, arozOS!");
}</textarea>
            </div>
        </div>
        <div class="status-bar">
            Ln 1, Col 1 | UTF-8 | <span id="langDisplay">Rust</span>
        </div>
    </div>
    
    <script>
        const editor = document.getElementById('editor');
        const filePath = document.getElementById('filePath');
        const language = document.getElementById('language');
        
        // Tab key support
        editor.addEventListener('keydown', function(e) {
            if (e.key === 'Tab') {
                e.preventDefault();
                const start = this.selectionStart;
                const end = this.selectionEnd;
                this.value = this.value.substring(0, start) + '    ' + this.value.substring(end);
                this.selectionStart = this.selectionEnd = start + 4;
            }
        });
        
        // Update status bar
        editor.addEventListener('keyup', updateCursorPos);
        editor.addEventListener('click', updateCursorPos);
        
        function updateCursorPos() {
            const text = editor.value.substring(0, editor.selectionStart);
            const lines = text.split('\n');
            const line = lines.length;
            const col = lines[lines.length - 1].length + 1;
            document.querySelector('.status-bar').innerHTML = 
                `Ln ${line}, Col ${col} | UTF-8 | ${language.options[language.selectedIndex].text}`;
        }
        
        language.addEventListener('change', updateCursorPos);
        
        function newFile() {
            editor.value = '// New file\n';
            filePath.value = '/home/user/untitled';
            updateCursorPos();
        }
        
        function openFile() {
            const path = prompt('Enter file path:', filePath.value);
            if (path) {
                filePath.value = path;
                // In real implementation, fetch file content from server
                editor.value = '// File loaded: ' + path + '\n';
            }
        }
        
        async function saveFile() {
            const path = filePath.value;
            const content = editor.value;
            try {
                const response = await fetch('/CodeStudio/api/save', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ path, content })
                });
                const result = await response.json();
                alert(result.success ? 'File saved!' : 'Error: ' + result.message);
            } catch (err) {
                alert('Save error: ' + err);
            }
        }
        
        updateCursorPos();
    </script>
</body>
</html>"#;
    Html(html.to_string())
}

/// Get Code Studio info
pub async fn api_info() -> Json<CodeStudioInfo> {
    Json(CodeStudioInfo {
        name: APP_NAME.to_string(),
        version: APP_VERSION.to_string(),
        description: APP_DESCRIPTION.to_string(),
        supported_languages: vec![
            "Rust".to_string(),
            "JavaScript".to_string(),
            "TypeScript".to_string(),
            "Python".to_string(),
            "Go".to_string(),
            "HTML".to_string(),
            "CSS".to_string(),
            "JSON".to_string(),
            "Markdown".to_string(),
        ],
    })
}

/// Save a file
pub async fn api_save(
    State(_state): State<Arc<RwLock<crate::AppState>>>,
    Json(req): Json<SaveFileRequest>,
) -> Json<SaveFileResponse> {
    // In a real implementation, this would write to the filesystem
    // with proper permission checks
    if req.path.is_empty() || req.content.is_empty() {
        return Json(SaveFileResponse {
            success: false,
            message: "Path and content are required".to_string(),
        });
    }
    
    // Validate path (prevent directory traversal)
    if req.path.contains("..") {
        return Json(SaveFileResponse {
            success: false,
            message: "Invalid path".to_string(),
        });
    }
    
    // Simulate successful save
    Json(SaveFileResponse {
        success: true,
        message: format!("Saved {} bytes to {}", req.content.len(), req.path),
    })
}

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
        .route("/api/save", axum::routing::post(api_save))
}
