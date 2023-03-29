use std::str::FromStr;

use diesel::{deserialize::FromSql, serialize::ToSql, sql_types::Text, sqlite::Sqlite, *};

use crate::schema::settings;
use crate::types::{Id, TextWrapper};

#[derive(Queryable)]
pub struct Setting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: TextWrapper<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

#[derive(Debug)]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl From<Theme> for TextWrapper<Theme> {
    fn from(val: Theme) -> Self {
        TextWrapper(val)
    }
}

impl AsRef<str> for Theme {
    fn as_ref(&self) -> &str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Theme::System),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err("Invalid theme".into()),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: TextWrapper<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = settings)]
pub struct PatchSetting {
    pub id: Id,
    pub user_id: Id,
    pub language: Option<String>,
    pub theme: Option<TextWrapper<Theme>>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}
