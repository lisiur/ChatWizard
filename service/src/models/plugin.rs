use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::plugins;
use crate::types::Id;
use crate::{ChatConfig, JsonWrapper};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig {
    pub chat_config: ChatConfig,
}

#[derive(Insertable)]
#[diesel(table_name = plugins)]
pub struct NewPlugin {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub code: Vec<u8>,
    pub config: JsonWrapper<PluginConfig>,
}

#[derive(AsChangeset)]
#[diesel(table_name = plugins)]
pub struct PatchPlugin {
    pub id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub code: Option<Vec<u8>>,
    pub config: Option<JsonWrapper<PluginConfig>>,
}

#[derive(Queryable, Selectable, Debug)]
pub struct Plugin {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub code: Vec<u8>,
    pub config: JsonWrapper<PluginConfig>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
