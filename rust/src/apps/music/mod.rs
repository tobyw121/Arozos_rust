//! Music Player Application
use axum::{Router, routing::get, response::Html, Json};
use serde::Serialize;

pub const APP_NAME: &str = "Music";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Music player and library manager";

#[derive(Debug, Serialize)]
pub struct MusicInfo { name: String, version: String, description: String }

pub async fn index() -> Html<String> {
    let html = r#"<!DOCTYPE html><html><head><title>Music - arozOS</title><meta charset="utf-8"><style>body{margin:0;padding:20px;font-family:Arial;background:linear-gradient(135deg,#1db954,#1ed760);color:white;min-height:100vh}.player{max-width:600px;margin:0 auto;text-align:center}.album-art{width:300px;height:300px;background:#333;border-radius:12px;margin:20px auto;display:flex;align-items:center;justify-content:center;font-size:72px}.controls{display:flex;justify-content:center;gap:15px;margin:20px 0}.btn{width:60px;height:60px;border-radius:50%;border:none;background:white;color:#1db954;font-size:24px;cursor:pointer;display:flex;align-items:center;justify-content-center}.btn:hover{transform:scale(1.1)}.progress{width:100%;height:6px;background:rgba(255,255,255,0.3);border-radius:3px;margin:20px 0}.progress-bar{height:100%;background:white;border-radius:3px;width:0%}.playlist{background:rgba(0,0,0,0.2);border-radius:12px;padding:15px;margin-top:20px}.track{display:flex;justify-content:space-between;padding:10px;border-radius:6px;cursor:pointer}.track:hover{background:rgba(255,255,255,0.1)}</style></head><body><div class="player"><h1>🎵 Music Player</h1><div class="album-art">🎶</div><h2 id="trackTitle">No track selected</h2><p id="artistName">Select a song to play</p><div class="progress"><div class="progress-bar" id="progressBar"></div></div><div class="controls"><button class="btn" onclick="prev()">⏮</button><button class="btn" onclick="playPause()" id="playBtn">▶</button><button class="btn" onclick="next()">⏭</button></div><div class="playlist" id="playlist"></div></div><script>let playing=false,tracks=[{title:"Sample Track",artist:"Artist"}];function playPause(){playing=!playing;document.getElementById('playBtn').textContent=playing?'⏸':'▶'}function prev(){}function next(){}function loadPlaylist(){document.getElementById('playlist').innerHTML=tracks.map(t=>`<div class="track"><span>${t.title}</span><span>${t.artist}</span></div>`).join('')}loadPlaylist()</script></body></html>"#;
    Html(html.to_string())
}

pub async fn api_info() -> Json<MusicInfo> {
    Json(MusicInfo { name: APP_NAME.to_string(), version: APP_VERSION.to_string(), description: APP_DESCRIPTION.to_string() })
}

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
}
