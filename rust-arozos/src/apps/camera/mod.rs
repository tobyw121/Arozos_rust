//! Camera Application
//! 
//! Provides camera access and photo capture functionality for arozOS.

use axum::{Router, routing::get, response::Html, Json};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "Camera";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Camera application for capturing photos and videos";

#[derive(Debug, Serialize)]
pub struct CameraInfo {
    name: String,
    version: String,
    description: String,
    supported: bool,
}

/// Main camera interface
pub async fn index() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Camera - arozOS</title>
    <meta charset="utf-8">
    <style>
        body { margin: 0; padding: 20px; font-family: Arial, sans-serif; background: #1a1a1a; color: white; }
        .camera-container { max-width: 800px; margin: 0 auto; text-align: center; }
        video { width: 100%; border-radius: 8px; background: #000; }
        .controls { margin-top: 20px; }
        button { padding: 12px 24px; margin: 8px; font-size: 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer; }
        button:hover { background: #0056b3; }
        canvas { display: none; }
        .gallery { margin-top: 20px; display: grid; grid-template-columns: repeat(auto-fill, minmax(150px, 1fr)); gap: 10px; }
        .gallery img { width: 100%; border-radius: 4px; cursor: pointer; }
    </style>
</head>
<body>
    <div class="camera-container">
        <h1>📷 Camera</h1>
        <video id="video" autoplay playsinline></video>
        <canvas id="canvas"></canvas>
        <div class="controls">
            <button onclick="startCamera()">Start Camera</button>
            <button onclick="capturePhoto()">📸 Capture</button>
            <button onclick="stopCamera()">Stop</button>
        </div>
        <div class="gallery" id="gallery"></div>
    </div>
    <script>
        let stream = null;
        const video = document.getElementById('video');
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');
        
        async function startCamera() {
            try {
                stream = await navigator.mediaDevices.getUserMedia({ video: true, audio: false });
                video.srcObject = stream;
            } catch (err) {
                alert('Camera access denied: ' + err);
            }
        }
        
        function capturePhoto() {
            if (!stream) return alert('Please start camera first');
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
            ctx.drawImage(video, 0, 0);
            const dataUrl = canvas.toDataURL('image/png');
            const img = document.createElement('img');
            img.src = dataUrl;
            img.onclick = () => downloadImage(dataUrl);
            document.getElementById('gallery').prepend(img);
        }
        
        function stopCamera() {
            if (stream) {
                stream.getTracks().forEach(track => track.stop());
                stream = null;
            }
        }
        
        function downloadImage(dataUrl) {
            const a = document.createElement('a');
            a.href = dataUrl;
            a.download = 'photo_' + Date.now() + '.png';
            a.click();
        }
    </script>
</body>
</html>"#;
    Html(html.to_string())
}

/// Get camera information
pub async fn api_info() -> Json<CameraInfo> {
    Json(CameraInfo {
        name: APP_NAME.to_string(),
        version: APP_VERSION.to_string(),
        description: APP_DESCRIPTION.to_string(),
        supported: true,
    })
}

/// List available cameras
pub async fn api_list_cameras() -> Json<Vec<String>> {
    // In a real implementation, this would enumerate system cameras
    Json(vec!["Default Camera".to_string()])
}

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
        .route("/api/cameras", get(api_list_cameras))
}
