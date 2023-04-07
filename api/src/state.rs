use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::extract::ws::{Message, WebSocket};
use chat_wizard_service::{DbConn, Id, commands::CommandExecutor};
use futures::{stream::SplitSink};
use tokio::sync::Mutex;

pub type ClientsMap = Arc<Mutex<HashMap<Id, Arc<Mutex<SplitSink<WebSocket, Message>>>>>>;
pub type UsersMap = Arc<Mutex<HashMap<Id, Arc<Mutex<HashSet<Id>>>>>>;

#[derive(Clone)]
pub struct AppState {
    pub conn: DbConn,
    pub clients: ClientsMap,
    pub users: UsersMap,
    pub executor: CommandExecutor,
}
