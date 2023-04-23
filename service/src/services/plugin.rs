use crate::{
    api::openai::chat::params::{OpenAIChatMessage, OpenAIChatParams, OpenAIChatRole},
    models::plugin::{NewPlugin, PatchPlugin, Plugin, PluginConfig},
    plugin::{RunningPlugin, RunningPluginState},
    repositories::{plugin::PluginRepo, setting::SettingRepo},
    result::Result,
    DbConn, Id, StreamContent, Error,
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

    pub async fn send_message(&self, prompt: &str) -> Result<String> {
        let user_message = OpenAIChatMessage {
            role: OpenAIChatRole::User,
            content: prompt.to_string(),
        };
        let setting = self.setting_repo.select_by_user_id(Id::local())?;
        let api = setting.create_openai_chat();
        let api_params = OpenAIChatParams {
            stream: true,
            model: "gpt-3.5".to_string(),
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
            Some(err) => return Err(Error::Unknown(err)),
            None => Ok(reply.unwrap())
        }
    }

    pub async fn execute(&self, id: Id) -> Result<()> {
        let plugin = self.get_plugin(id)?;
        let state = RunningPluginState::new(self.clone());
        let mut running_plugin = RunningPlugin::init(&plugin.code, state).await?;
        running_plugin.run().await?;

        Ok(())
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
