use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::extract::ws::{Message, WebSocket};
use chat_wizard_service::{DbConn, Id};
use futures::{stream::SplitSink, SinkExt};
use tokio::sync::Mutex;

pub type ClientsMap = Arc<Mutex<HashMap<Id, Arc<Mutex<SplitSink<WebSocket, Message>>>>>>;
pub type UsersMap = Arc<Mutex<HashMap<Id, Arc<Mutex<HashSet<Id>>>>>>;

#[derive(Clone)]
pub struct AppState {
    pub conn: DbConn,
    pub clients: ClientsMap,
    pub users: UsersMap,
}

impl AppState {
    pub async fn send_message(&self, user_id: Id, message: Message) {
        let users = self.users.lock().await;
        let user_clients = users.get(&user_id);
        if let Some(user_clients) = user_clients {
            let user_clients = user_clients.lock().await;
            // TODO: send concurrent
            for client in user_clients.iter() {
                let clients = self.clients.lock().await;
                if let Some(client) = clients.get(client) {
                    let mut client = client.lock().await;
                    client.send(message.clone()).await.unwrap();
                }
            }
        }
    }
}
