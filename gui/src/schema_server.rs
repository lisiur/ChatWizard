use std::net::SocketAddr;

use axum::{extract::State, routing::get, Json, Router};
use tauri::AppHandle;

use crate::window::show_or_create_main_window;

#[derive(Clone)]
pub struct SchemaState {
    pub app_handle: AppHandle,
}

pub async fn serve(port: u16, app_handle: AppHandle) {
    let state = SchemaState { app_handle };

    let app = Router::new()
        .route("/open", get(open_handler))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn open_handler(State(state): State<SchemaState>) -> Json<()> {
    let app_handle = state.app_handle;
    show_or_create_main_window(&app_handle, "index.html")
        .await
        .unwrap();
    Json(())
}
