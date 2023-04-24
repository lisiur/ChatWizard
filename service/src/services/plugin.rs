use crate::{
    api::openai::chat::params::{OpenAIChatMessage, OpenAIChatParams, OpenAIChatRole},
    models::plugin::{InstalledPlugin, NewPlugin, PatchPlugin, Plugin, PluginConfig},
    plugin::{RunningPlugin, RunningPluginState},
    repositories::{plugin::PluginRepo, setting::SettingRepo},
    result::Result,
    DbConn, Error, Id, StreamContent,
};
use futures::StreamExt;

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

    pub fn all_plugins(&self) -> Result<Vec<InstalledPlugin>> {
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

    pub fn get_plugin_by_name(&self, name: &str) -> Result<Plugin> {
        self.plugin_repo.select_by_name(name)
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

    pub async fn send_message(&self, prompt: &str) -> Result<String> {
        let user_message = OpenAIChatMessage {
            role: OpenAIChatRole::User,
            content: prompt.to_string(),
        };
        let setting = self.setting_repo.select_by_user_id(Id::local())?;
        let api = setting.create_openai_chat();
        let api_params = OpenAIChatParams {
            stream: true,
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![user_message],
            ..Default::default()
        };

        let mut reply = Some(String::new());
        let mut error = Option::<String>::None;
        let stream = api.send_message(api_params).await;
        match stream {
            Ok(mut stream) => {
                while let Some(content) = stream.next().await {
                    match &content {
                        StreamContent::Data(data) => match &mut reply {
                            Some(reply) => reply.push_str(data),
                            None => unreachable!(),
                        },
                        StreamContent::Done => {
                            break;
                        }
                        StreamContent::Error(err) => {
                            error = Some(err.to_string());
                            break;
                        }
                    }
                }
                drop(stream);
            }
            Err(err) => error = Some(err.to_string()),
        }

        match error {
            Some(err) => Err(Error::Unknown(err)),
            None => Ok(reply.unwrap()),
        }
    }

    pub async fn execute_by_name(&self, name: &str) -> Result<()> {
        let plugin = self.get_plugin_by_name(name)?;

        self.execute(plugin).await
    }

    pub async fn execute(&self, plugin: Plugin) -> Result<()> {
        let state = RunningPluginState::new(self.clone());
        let mut running_plugin = RunningPlugin::init(&plugin.code, state).await?;
        running_plugin.run().await?;

        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreatePluginPayload {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub code: Vec<u8>,
    pub config: PluginConfig,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdatePluginPayload {
    pub id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub code: Option<Vec<u8>>,
    pub config: Option<PluginConfig>,
}
