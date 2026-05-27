//! Web Browser Application
//! 
//! Provides a web browsing interface within the arozOS desktop environment.

use axum::{Router, routing::get, response::Html, extract::Query};
use serde::Deserialize;
use std::collections::HashMap;

/// Application metadata
pub const APP_NAME: &str = "Browser";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Web Browser for arozOS";

#[derive(Debug, Deserialize)]
pub struct BrowserParams {
    url: Option<String>,
    title: Option<String>,
}

/// Main browser interface handler
pub async fn index() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Browser - arozOS</title>
    <meta charset="utf-8">
    <style>
        body { margin: 0; padding: 0; font-family: Arial, sans-serif; }
        .browser-container { display: flex; flex-direction: column; height: 100vh; }
        .address-bar { display: flex; padding: 8px; background: #f0f0f0; border-bottom: 1px solid #ccc; }
        .address-bar input { flex: 1; padding: 6px; border: 1px solid #ccc; border-radius: 4px; }
        .address-bar button { margin-left: 8px; padding: 6px 12px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }
        .iframe-container { flex: 1; position: relative; }
        iframe { width: 100%; height: 100%; border: none; }
    </style>
</head>
<body>
    <div class="browser-container">
        <div class="address-bar">
            <input type="text" id="urlInput" placeholder="Enter URL..." value="https://">
            <button onclick="navigate()">Go</button>
            <button onclick="goBack()">Back</button>
            <button onclick="goForward()">Forward</button>
            <button onclick="refresh()">Refresh</button>
        </div>
        <div class="iframe-container">
            <iframe id="browserFrame" src="about:blank"></iframe>
        </div>
    </div>
    <script>
        function navigate() {
            let url = document.getElementById('urlInput').value;
            if (!url.startsWith('http')) {
                url = 'https://' + url;
            }
            document.getElementById('browserFrame').src = url;
        }
        function goBack() { history.back(); }
        function goForward() { history.forward(); }
        function refresh() { document.getElementById('browserFrame').contentWindow.location.reload(); }
        document.getElementById('urlInput').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') navigate();
        });
    </script>
</body>
</html>"#;
    Html(html.to_string())
}

/// API endpoint to get browser info
pub async fn api_info() -> String {
    format!(r#"{{"name":"{}","version":"{}","description":"{}"}}"#, 
            APP_NAME, APP_VERSION, APP_DESCRIPTION)
}

/// Create router for browser app
pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
}
