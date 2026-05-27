//! Memo Application - Quick notes and memos
use axum::{Router, routing::get, response::Html, Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub const APP_NAME: &str = "Memo";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "Quick notes and memos";

#[derive(Debug, Serialize)]
pub struct MemoInfo { name: String, version: String, description: String }

#[derive(Debug, Serialize, Clone)]
pub struct Memo { id: u64, title: String, content: String, created: u64 }

#[derive(Debug, Deserialize)]
pub struct CreateMemo { title: String, content: String }

pub async fn index() -> Html<String> {
    let html = r#"<!DOCTYPE html><html><head><title>Memo - arozOS</title><meta charset="utf-8"><style>body{margin:0;padding:20px;font-family:Arial;background:#f5f5f5}.container{max-width:800px;margin:0 auto}.header{display:flex;justify-content:space-between;align-items:center;margin-bottom:20px}button{padding:10px 20px;background:#007bff;color:white;border:none;border-radius:6px;cursor:pointer}.memo-list{display:grid;gap:10px}.memo-item{background:white;padding:15px;border-radius:8px;box-shadow:0 2px 4px rgba(0,0,0,0.1);cursor:pointer}.memo-item:hover{box-shadow:0 4px 8px rgba(0,0,0,0.15)}.modal{display:none;position:fixed;top:0;left:0;width:100%;height:100%;background:rgba(0,0,0,0.5)}.modal-content{background:white;margin:5% auto;padding:20px;border-radius:8px;max-width:600px}input,textarea{width:100%;margin:10px 0;padding:10px;border:1px solid #ddd;border-radius:4px}textarea{height:300px;resize:vertical}</style></head><body><div class="container"><div class="header"><h1>📝 Memos</h1><button onclick="newMemo()">+ New Memo</button></div><div class="memo-list" id="memoList"></div></div><div class="modal" id="modal"><div class="modal-content"><h2 id="modalTitle">New Memo</h2><input type="text" id="memoTitle" placeholder="Title"><textarea id="memoContent" placeholder="Content..."></textarea><div style="display:flex;gap:10px;justify-content:flex-end"><button onclick="closeModal()" style="background:#6c757d">Cancel</button><button onclick="saveMemo()">Save</button><button onclick="deleteMemo()" style="background:#dc3545;display:none" id="deleteBtn">Delete</button></div></div></div><script>let memos=[],currentId=null;async function loadMemos(){const r=await fetch('/Memo/api/list');memos=await r.json();render()}function render(){document.getElementById('memoList').innerHTML=memos.map(m=>`<div class="memo-item" onclick="editMemo(${m.id})"><h3>${m.title}</h3><p>${m.content.substring(0,100)}...</p><small>${new Date(m.created*1000).toLocaleString()}</small></div>`).join('')}function newMemo(){currentId=null;document.getElementById('memoTitle').value='';document.getElementById('memoContent').value='';document.getElementById('modalTitle').textContent='New Memo';document.getElementById('deleteBtn').style.display='none';document.getElementById('modal').style.display='block'}function editMemo(id){const m=memos.find(x=>x.id===id);if(m){currentId=id;document.getElementById('memoTitle').value=m.title;document.getElementById('memoContent').value=m.content;document.getElementById('modalTitle').textContent='Edit Memo';document.getElementById('deleteBtn').style.display='inline-block';document.getElementById('modal').style.display='block'}}function closeModal(){document.getElementById('modal').style.display='none'}async function saveMemo(){const title=document.getElementById('memoTitle').value,content=document.getElementById('memoContent').value;if(!title)return alert('Title required');await fetch('/Memo/api/save',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({id:currentId,title,content})});closeModal();loadMemos()}async function deleteMemo(){if(currentId&&confirm('Delete this memo?')){await fetch('/Memo/api/delete',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({id:currentId})});closeModal();loadMemos()}}loadMemos()</script></body></html>"#;
    Html(html.to_string())
}

pub async fn api_info() -> Json<MemoInfo> {
    Json(MemoInfo { name: APP_NAME.to_string(), version: APP_VERSION.to_string(), description: APP_DESCRIPTION.to_string() })
}

pub async fn api_list(State(_state): State<Arc<RwLock<crate::AppState>>>) -> Json<Vec<Memo>> {
    Json(vec![]) // Would load from database in real implementation
}

pub async fn api_save(State(_state): State<Arc<RwLock<crate::AppState>>>, Json(req): Json<CreateMemo>) -> Json<Memo> {
    Json(Memo { id: 1, title: req.title, content: req.content, created: 0 })
}

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
        .route("/api/list", get(api_list))
        .route("/api/save", axum::routing::post(api_save))
}
