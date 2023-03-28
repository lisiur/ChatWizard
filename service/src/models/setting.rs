use diesel::{deserialize::FromSql, serialize::ToSql, sql_types::Text, sqlite::Sqlite, *};

use crate::schema::settings;
use crate::types::Id;

#[derive(Queryable)]
pub struct Setting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: String,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

#[derive(AsExpression, Debug)]
#[diesel(sql_type = Text)]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::System => "system".to_string(),
            Theme::Light => "light".to_string(),
            Theme::Dark => "dark".to_string(),
        }
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

impl FromSql<Text, Sqlite> for Theme {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        match String::from_utf8(bytes)?.as_ref() {
            "system" => Ok(Theme::System),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err("Invalid theme".into()),
        }
    }
}

impl ToSql<Text, Sqlite> for Theme {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        <str as ToSql<Text, Sqlite>>::to_sql(self.as_ref(), out)
    }
}

#[derive(Insertable)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: Theme,
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
    pub theme: Option<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}
