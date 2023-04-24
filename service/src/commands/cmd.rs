use crate::{
    models::{
        chat_log::ChatLog, chat_model::ChatModel, plugin::InstalledPlugin,
        prompt_source::PromptSource,
    },
    result::Result,
    services::{chat::*, plugin::PluginService},
    services::{plugin_market::InstallMarketPluginPayload, setting::*},
    services::{plugin_market::MarketPlugin, prompt_market::*},
    services::{plugin_market::PluginMarketService, prompt::*},
    Chat, ChatConfig, CursorQueryResult, DbConn, Id, Prompt, PromptIndex, Setting, StreamContent,
    Theme,
};
use serde::Deserialize;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::oneshot::{self, Sender};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectCommand;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

impl LoginCommand {
    pub fn exec(self, _conn: &DbConn) -> Result<Id> {
        Ok(Id::local())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChatCommand {
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
}

impl NewChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Id> {
        let chat_service = ChatService::new(conn.clone());

        let title = self.title.as_deref().unwrap_or("New Chat");
        let chat_id = chat_service.create_chat(CreateChatPayload {
            title: title.to_string(),
            user_id: Id::local(),
            prompt_id: self.prompt_id,
            vendor: "openai".to_string(),
            config: ChatConfig::default(),
        })?;

        Ok(chat_id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatCommand {
    pub id: Id,
}

impl GetChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Chat> {
        let chat_service = ChatService::new(conn.clone());

        let chat = chat_service.get_chat(self.id)?;

        Ok(chat)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllChatsExceptCasualCommand {
    #[serde(default)]
    user_id: Id,
}

impl AllChatsExceptCasualCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<Chat>> {
        let chat_service = ChatService::new(conn.clone());

        let result = chat_service.get_all_chats_except_casual(self.user_id)?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CasualChatCommand {
    #[serde(default)]
    user_id: Id,
}

impl CasualChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Chat> {
        let chat_service = ChatService::new(conn.clone());

        let result = chat_service.get_casual_chat(self.user_id)?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChatArchiveCommand {
    pub chat_id: Id,
}

impl SetChatArchiveCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.set_chat_archive(self.chat_id)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChatStickCommand {
    pub chat_id: Id,
    pub stick: bool,
}

impl SetChatStickCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.set_chat_stick(Id::local(), self.chat_id, self.stick)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveStickChatCommand {
    pub from: Id,
    pub to: Id,
}

impl MoveStickChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.move_stick_chat(MoveChatPayload {
            user_id: Id::local(),
            from: self.from,
            to: self.to,
        })?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveNonStickChatCommand {
    pub from: Id,
    pub to: Id,
}

impl MoveNonStickChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.move_non_stick_chat(MoveChatPayload {
            user_id: Id::local(),
            from: self.from,
            to: self.to,
        })?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatCommand {
    payload: UpdateChatPayload,
}

impl UpdateChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.update_chat(self.payload)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveChatPromptCommand {
    id: Id,
}

impl RemoveChatPromptCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.remove_prompt(self.id)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadChatLogByCursorCommand {
    pub chat_id: Id,
    pub size: i64,
    pub cursor: Option<Id>,
}

impl LoadChatLogByCursorCommand {
    pub fn exec(self, conn: &DbConn) -> Result<CursorQueryResult<ChatLog>> {
        let chat_service = ChatService::new(conn.clone());

        let result = chat_service.get_chat_logs_by_cursor(GetChatLogByCursorPayload {
            cursor: self.cursor,
            size: self.size,
            user_id: Id::local(),
            chat_id: Some(self.chat_id),
            ..Default::default()
        })?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteChatCommand {
    pub chat_id: Id,
}

impl DeleteChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.delete_chat(DeleteChatPayload { id: self.chat_id })?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatLogCommand {
    pub id: Id,
    pub content: String,
}

impl UpdateChatLogCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.update_chat_log(UpdateChatLogPayload {
            id: self.id,
            content: self.content,
        })?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteChatLogCommand {
    pub log_id: Id,
}

impl DeleteChatLogCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let chat_service = ChatService::new(conn.clone());

        chat_service.delete_chat_log(self.log_id)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageCommand {
    pub chat_id: Id,
    pub message: String,
}

impl SendMessageCommand {
    pub async fn exec(
        self,
        conn: &DbConn,
    ) -> Result<(Receiver<StreamContent>, Sender<()>, Id, Id)> {
        let chat_service = ChatService::new(conn.clone());

        let (sender, receiver) = mpsc::channel::<StreamContent>(20);
        let (stop_sender, stop_receiver) = oneshot::channel::<()>();
        let (message_id, reply_id, _) = chat_service
            .send_message(
                SendMessagePayload {
                    chat_id: self.chat_id,
                    message: self.message,
                },
                sender,
                stop_receiver,
            )
            .await?;
        Ok((receiver, stop_sender, message_id, reply_id))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendMessageCommand {
    pub message_id: Id,
}

impl ResendMessageCommand {
    pub async fn exec(
        self,
        conn: &DbConn,
    ) -> Result<(Receiver<StreamContent>, Sender<()>, Id, Id)> {
        let chat_service = ChatService::new(conn.clone());

        let (sender, receiver) = mpsc::channel::<StreamContent>(20);
        let (stop_sender, stop_receiver) = oneshot::channel::<()>();
        let (message_id, reply_id, _) = chat_service
            .resend_message(
                ResendMessagePayload {
                    id: self.message_id,
                },
                sender,
                stop_receiver,
            )
            .await?;
        Ok((receiver, stop_sender, message_id, reply_id))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopReplyCommand {
    pub message_id: Id,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatModelsCommand;

impl GetChatModelsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<ChatModel>> {
        let chat_service = ChatService::new(conn.clone());

        let records = chat_service.get_chat_models()?;

        Ok(records)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPromptsCommand;

impl AllPromptsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<PromptIndex>> {
        let prompt_service = PromptService::new(conn.clone());

        let result = prompt_service.search_prompts(SearchPromptPayload::default())?;

        Ok(result.records)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadPromptCommand {
    pub id: Id,
}

impl LoadPromptCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Prompt> {
        let prompt_service = PromptService::new(conn.clone());

        let result = prompt_service.get_prompt(self.id)?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePromptCommand {
    pub name: String,
    pub content: String,
}

impl CreatePromptCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Id> {
        let prompt_service = PromptService::new(conn.clone());

        let id = prompt_service.create_prompt(CreatePromptPayload {
            name: self.name,
            content: self.content,
            user_id: Id::local(),
        })?;

        Ok(id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePromptCommand {
    pub payload: UpdatePromptPayload,
}

impl UpdatePromptCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let prompt_service = PromptService::new(conn.clone());

        prompt_service.update_prompt(self.payload)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePromptCommand {
    pub id: Id,
}

impl DeletePromptCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let prompt_service = PromptService::new(conn.clone());

        prompt_service.delete_prompt(self.id)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPromptSourcesCommand;

impl GetPromptSourcesCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<PromptSource>> {
        let prompt_market_service = PromptMarketService::new(conn.clone());

        let result = prompt_market_service.get_prompt_sources()?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPromptSourcePromptsCommand {
    pub source_id: Id,
}

impl GetPromptSourcePromptsCommand {
    pub async fn exec(self, conn: &DbConn) -> Result<Vec<MarketPrompt>> {
        let prompt_market_service = PromptMarketService::new(conn.clone());

        let result = prompt_market_service
            .get_prompt_source_prompts(self.source_id)
            .await?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallMarketPromptCommand {
    pub name: String,
    pub content: String,
}

impl InstallMarketPromptCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Id> {
        let prompt_market_service = PromptMarketService::new(conn.clone());

        let id = prompt_market_service.install_market_prompt(InstallMarketPromptPayload {
            prompt: MarketPrompt {
                name: self.name,
                content: self.content,
            },
            user_id: Id::local(),
        })?;

        Ok(id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallMarketPromptAndCreateChatCommand {
    pub name: String,
    pub content: String,
}

impl InstallMarketPromptAndCreateChatCommand {
    pub fn exec(self, conn: &DbConn) -> Result<(Id, Id)> {
        let prompt_market_service = PromptMarketService::new(conn.clone());

        let (prompt_id, chat_id) = prompt_market_service.install_market_prompt_and_create_chat(
            InstallMarketPromptPayload {
                prompt: MarketPrompt {
                    name: self.name,
                    content: self.content,
                },
                user_id: Id::local(),
            },
        )?;

        Ok((prompt_id, chat_id))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarketPluginsCommand;

impl GetMarketPluginsCommand {
    pub async fn exec(self, conn: &DbConn) -> Result<Vec<MarketPlugin>> {
        let plugin_market_service = PluginMarketService::new(conn.clone());

        let result = plugin_market_service.get_market_plugins().await?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllMarketPluginsCommand;

impl GetAllMarketPluginsCommand {
    pub async fn exec(self, conn: &DbConn) -> Result<Vec<MarketPlugin>> {
        let plugin_service = PluginMarketService::new(conn.clone());

        let result = plugin_service.get_market_plugins().await?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllInstalledPluginsCommand;

impl GetAllInstalledPluginsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<InstalledPlugin>> {
        let plugin_service = PluginService::new(conn.clone());

        let result = plugin_service.all_plugins()?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallPluginCommand {
    pub market_plugin: MarketPlugin,
}

impl InstallPluginCommand {
    pub async fn exec(self, conn: &DbConn) -> Result<Id> {
        let plugin_market_service = PluginMarketService::new(conn.clone());

        let plugin_id = plugin_market_service
            .install_market_plugin(InstallMarketPluginPayload {
                plugin: self.market_plugin,
                user_id: Id::local(),
            })
            .await?;

        Ok(plugin_id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UninstallPluginCommand {
    pub id: Id,
}

impl UninstallPluginCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let plugin_service = PluginService::new(conn.clone());

        plugin_service.delete_plugin(self.id)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettingsCommand;

impl GetSettingsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Setting> {
        let setting_service = SettingService::new(conn.clone());

        let result = setting_service.get_setting(Id::local())?;

        Ok(result)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetThemeCommand;

impl GetThemeCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Theme> {
        let setting_service = SettingService::new(conn.clone());

        let result = setting_service.get_setting(Id::local())?;

        Ok(result.theme.0)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingCommand {
    pub payload: UpdateSettingPayload,
}

impl UpdateSettingCommand {
    pub fn exec(self, conn: &DbConn) -> Result<()> {
        let setting_service = SettingService::new(conn.clone());

        setting_service.update_setting(self.payload)?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocaleCommand;

impl GetLocaleCommand {
    pub fn exec(self, conn: &DbConn) -> Result<String> {
        let setting_service = SettingService::new(conn.clone());

        let result = setting_service.get_setting(Id::local())?;

        Ok(result.language)
    }
}
