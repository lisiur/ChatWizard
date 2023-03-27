use chrono::NaiveDateTime;
use diesel::Queryable;

use crate::types::Id;

#[derive(Queryable)]
pub struct ChatLog {
    id: Id,
    chat_id: Id,
    role: Role,
    message: String,
    model: String,
    tokens: usize,
    cost: f32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub enum Role {
    System,
    Assistant,
    User,
}
