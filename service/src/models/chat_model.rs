use chrono::NaiveDateTime;

use crate::schema::chat_models;
use crate::types::Id;
use diesel::*;

#[derive(Queryable)]
pub struct ChatModel {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub unit: String,
    pub vendor: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ChatModel {
    pub fn calc_cost(&self, tokens: usize) -> f32 {
        self.price * tokens as f32 / 1000.0
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = chat_models)]

pub struct NewChatModel {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub unit: String,
    pub vendor: String,
}
