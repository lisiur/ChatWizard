use std::sync::Arc;

use axum::extract::ws::{WebSocket, Message};
use chat_wizard_service::Id;
use futures::StreamExt;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::state::ClientsMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "payload")]
pub enum SocketMessage {}

pub async fn handle_socket(socket: WebSocket, client_id: Id, clients: ClientsMap) {
    log::debug!("New websocket connection: {}", client_id);

    let (sender, mut receiver) = socket.split();

    clients
        .lock()
        .await
        .insert(client_id, Arc::new(Mutex::new(sender)));

    let clients = clients.clone();
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    if let Ok(message) = serde_json::from_str::<SocketMessage>(&text) {
                        println!("Received message: {:?}", message);
                    }
                }
                Message::Close(_) => {
                    log::debug!("Websocket connection closed: {}", client_id);
                    clients.lock().await.remove(&client_id);
                }
                _ => {}
            }
        } else {
            return;
        }
    }
}