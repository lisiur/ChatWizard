mod dist;
mod error;
mod result;
mod state;
mod websocket;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post, Router};
use axum::Json;
use chat_wizard_service::commands::{CommandEvent, CommandExecutor};
use dist::StaticFile;

use chat_wizard_service::{DbConn, Id};
pub use error::Error;
use futures::SinkExt;
pub use result::IntoResultResponse;
pub use result::Result;

use axum::extract::ws::{Message, WebSocketUpgrade};
use serde::{Deserialize, Serialize};
use serde_json::json;
use state::{AppState, ClientsMap, UsersMap};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use websocket::handle_socket;

pub async fn app(port: u16, conn: DbConn) {
    let state = AppState {
        conn,
        clients: Arc::new(Mutex::new(HashMap::new())),
        users: Arc::new(Mutex::new(HashMap::new())),
        executor: CommandExecutor::new(),
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

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Listening on http://127.0.0.1:{}", port);
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandPayload {
    pub command: String,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub user_id: Id,
}

#[axum::debug_handler]
async fn command_handler(
    State(state): State<AppState>,
    Json(params): Json<CommandPayload>,
) -> impl IntoResponse {
    let user_id = Id::local();
    let users = state.users.clone();
    let clients = state.clients.clone();
    let executor = state.executor.clone();
    let send = move |event: CommandEvent| {
        let payload = json!({
            "id": event.name,
            "payload": event.payload,
        });
        let message = Message::Text(serde_json::to_string(&payload).unwrap());

        let users = users.clone();
        let clients = clients.clone();
        async move {
            send_message(users, clients, user_id, message).await;
            Ok(())
        }
    };

    let result = executor
        .exec_command(params.command, params.payload, &state.conn, send)
        .await
        .map_err(Error::Service);

    result.into_response()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketUpgradeQuery {
    client_id: Id,
    #[serde(default)]
    user_id: Id,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WebSocketUpgradeQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            query.user_id,
            query.client_id,
            state.users,
            state.clients,
        )
    })
}

pub async fn send_message(users: UsersMap, clients: ClientsMap, user_id: Id, message: Message) {
    let users = users.lock().await;
    let user_clients = users.get(&user_id);
    if let Some(user_clients) = user_clients {
        let user_clients = user_clients.lock().await;
        // TODO: send concurrent
        for client in user_clients.iter() {
            let clients = clients.lock().await;
            if let Some(client) = clients.get(client) {
                let mut client = client.lock().await;
                client.send(message.clone()).await.unwrap();
            }
        }
    }
}
