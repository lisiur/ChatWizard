mod command;
mod dist;
mod error;
mod result;
mod state;
mod websocket;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, Method, Uri};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post, Router};
use axum::Json;
use command::{handle_command, CommandPayload};
use dist::StaticFile;

use chat_wizard_service::{DbConn, Id};
pub use error::Error;
pub use result::IntoResultResponse;
pub use result::Result;

use axum::extract::ws::WebSocketUpgrade;
use serde::{Deserialize, Serialize};
use state::AppState;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use websocket::handle_socket;

pub async fn app(port: u16, conn: DbConn) {
    let state = AppState {
        conn,
        clients: Arc::new(Mutex::new(HashMap::new())),
        users: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/assets/*file", get(static_handler))
        .nest(
            "/api",
            Router::new()
                .route("/command", post(command_handler))
                .route("/ws", get(ws_handler)),
        )
        .fallback_service(get(not_found_handler))
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();

    StaticFile(path)
}

async fn not_found_handler() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

async fn command_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(params): Json<CommandPayload>,
) -> impl IntoResponse {
    let client_id = headers
        .get("x-client-id")
        .map(|v| Id::from(v.to_str().unwrap()))
        .unwrap();
    handle_command(params, state, client_id)
        .await
        .into_response()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketUpgradeQuery {
    client_id: Id,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WebSocketUpgradeQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, query.client_id, state.clients))
}
