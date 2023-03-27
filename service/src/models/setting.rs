use diesel::Queryable;

use crate::types::Id;

#[derive(Queryable)]
pub struct Setting {
    pub id: Id,
    pub language: String,
    pub theme: String,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}

pub enum Theme {
    System,
    Light,
    Dark,
}
