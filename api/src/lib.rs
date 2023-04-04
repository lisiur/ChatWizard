mod error;
mod result;

use std::net::SocketAddr;

use axum::http::Method;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

pub use error::Error;
pub use result::Result;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use tower_http::cors::{Any, CorsLayer};

pub async fn app(port: u16) {
    let app = Router::new().route("/ws", get(ws_handler)).layer(
        CorsLayer::new().allow_origin(Any).allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        if socket
                            .send(Message::Text(format!("Echo from backend: {}", t)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    Message::Close(_) => {
                        return;
                    }
                    _ => {}
                }
            } else {
                return;
            }
        }
    }
}
