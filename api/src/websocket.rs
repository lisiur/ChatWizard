use std::{collections::HashSet, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use chat_wizard_service::Id;
use futures::StreamExt;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::state::{ClientsMap, UsersMap};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "payload")]
pub enum SocketMessage {
    Connect,
}

pub async fn handle_socket(
    socket: WebSocket,
    user_id: Id,
    client_id: Id,
    users: UsersMap,
    clients: ClientsMap,
) {
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
                        match message {
                            SocketMessage::Connect => {
                                handle_connect(user_id, client_id, users.clone()).await;
                            }
                        }
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

async fn handle_connect(user_id: Id, client_id: Id, users: UsersMap) {
    let users = users.clone();
    let mut map = users.lock().await;
    map.entry(user_id)
        .or_insert_with(|| Arc::new(Mutex::new(HashSet::new())));
    map.get(&user_id).unwrap().lock().await.insert(client_id);
}
