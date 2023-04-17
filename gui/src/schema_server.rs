use std::net::SocketAddr;

use axum::{Router, routing::post, Json};
use tauri::AppHandle;

use crate::window::show_or_create_main_window;

#[derive(Clone)]
pub struct SchemaState {
    pub app_handle: AppHandle,
}

pub async fn serve(port: u16, app_handle: AppHandle) {
    let state = SchemaState { app_handle };

    let app = Router::new().route("/", post(handler)).with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(axum::extract::State(state): axum::extract::State<SchemaState>) -> Json<()> {
    let app_handle = state.app_handle;
    show_or_create_main_window(&app_handle, "index.html")
        .await
        .unwrap();
    Json(())
}
