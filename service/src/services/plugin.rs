use crate::{
    models::plugin::{NewPlugin, PatchPlugin, Plugin, PluginConfig},
    repositories::{plugin::PluginRepo, setting::SettingRepo},
    result::Result,
    DbConn, Id,
};

#[derive(Clone)]
pub struct PluginService {
    #[allow(unused)]
    conn: DbConn,
    plugin_repo: PluginRepo,
    setting_repo: SettingRepo,
}

impl From<DbConn> for PluginService {
    fn from(conn: DbConn) -> Self {
        Self::new(conn)
    }
}

impl PluginService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            plugin_repo: PluginRepo::new(conn.clone()),
            setting_repo: SettingRepo::new(conn.clone()),
            conn,
        }
    }

    pub fn all_plugins(&self) -> Result<Vec<Plugin>> {
        self.plugin_repo.select_all()
    }

    pub fn create_plugin(&self, payload: CreatePluginPayload) -> Result<Id> {
        let plugin_id = Id::random();

        let new_plugin = NewPlugin {
            id: plugin_id,
            name: payload.name,
            description: payload.description,
            version: payload.version,
            author: payload.author,
            code: payload.code,
            config: payload.config.into(),
        };

        self.plugin_repo.insert(new_plugin)?;

        Ok(plugin_id)
    }

    pub fn get_plugin(&self, id: Id) -> Result<Plugin> {
        self.plugin_repo.select_by_id(id)
    }

    pub fn update_plugin(&self, payload: UpdatePluginPayload) -> Result<()> {
        let patch_plugin = PatchPlugin {
            id: payload.id,
            name: payload.name,
            description: payload.description,
            version: payload.version,
            author: payload.author,
            code: payload.code,
            config: payload.config.map(Into::into),
        };

        self.plugin_repo.update(patch_plugin)?;

        Ok(())
    }

    pub fn delete_plugin(&self, id: Id) -> Result<()> {
        self.plugin_repo.delete_by_id(id)?;

        Ok(())
    }

    pub async fn execute(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreatePluginPayload {
    name: String,
    description: String,
    version: String,
    author: String,
    code: Vec<u8>,
    config: PluginConfig,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdatePluginPayload {
    id: Id,
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    author: Option<String>,
    code: Option<Vec<u8>>,
    config: Option<PluginConfig>,
}
