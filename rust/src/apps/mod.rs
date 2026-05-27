//! ArozOS Desktop Applications
//! 
//! This module contains all the desktop applications that run on top of the arozOS platform.
//! Each application is a self-contained module with its own handlers, routes, and business logic.

pub mod browser;
pub mod camera;
pub mod clock;
pub mod code_studio;
pub mod md_editor;
pub mod memo;
pub mod music;
pub mod notepad;
pub mod office_viewer;
pub mod pdf_viewer;
pub mod paint;
pub mod photo;
pub mod recorder;
pub mod speedtest;
pub mod timer;
pub mod video;
pub mod web_builder;
pub mod web_downloader;

use axum::{Router, routing::get};
use crate::AppState;

/// Register all application routes
pub fn register_apps(app: Router<AppState>) -> Router<AppState> {
    app
        .nest("/Browser", browser::routes())
        .nest("/Camera", camera::routes())
        .nest("/Clock", clock::routes())
        .nest("/CodeStudio", code_studio::routes())
        .nest("/MDEditor", md_editor::routes())
        .nest("/Memo", memo::routes())
        .nest("/Music", music::routes())
        .nest("/NotepadA", notepad::routes())
        .nest("/OfficeViewer", office_viewer::routes())
        .nest("/PDFViewer", pdf_viewer::routes())
        .nest("/Paint", paint::routes())
        .nest("/Photo", photo::routes())
        .nest("/Recorder", recorder::routes())
        .nest("/Speedtest", speedtest::routes())
        .nest("/Timer", timer::routes())
        .nest("/Video", video::routes())
        .nest("/WebBuilder", web_builder::routes())
        .nest("/WebDownloader", web_downloader::routes())
}
