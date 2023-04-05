use crate::{
    models::{chat_log::ChatLog, chat_model::ChatModel, prompt_source::PromptSource},
    result::Result,
    services::prompt_market::{InstallMarketPromptPayload, MarketPrompt, PromptMarketService},
    Chat, ChatConfig, ChatService, CreateChatPayload, CreatePromptPayload, CursorQueryResult,
    DbConn, DeleteChatPayload, GetChatLogByCursorPayload, Id, MoveChatPayload, Prompt, PromptIndex,
    PromptService, ResendMessagePayload, SearchChatPayload, SearchPromptPayload,
    SendMessagePayload, Setting, SettingService, StreamContent, Theme, UpdateChatPayload,
    UpdatePromptPayload, UpdateSettingPayload,
};
use serde::{Deserialize};
use tokio::sync::mpsc::{self, Receiver};

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
pub struct AllChatsCommand;

impl AllChatsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<Chat>> {
        let chat_service = ChatService::new(conn.clone());

        let result = chat_service.search_chats(SearchChatPayload::default())?;

        Ok(result.records)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllNonStickChatsCommand;

impl AllNonStickChatsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<Chat>> {
        let chat_service = ChatService::new(conn.clone());

        let result = chat_service.get_non_stick_chats(SearchChatPayload::default())?;

        Ok(result.records)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllStickChatsCommand;

impl AllStickChatsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<Chat>> {
        let chat_service = ChatService::new(conn.clone());

        let records = chat_service.get_stick_chats(SearchChatPayload::default())?;

        Ok(records)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllArchiveChatsCommand;

impl AllArchiveChatsCommand {
    pub fn exec(self, conn: &DbConn) -> Result<Vec<Chat>> {
        let chat_service = ChatService::new(conn.clone());

        let records = chat_service.get_archive_chats(SearchChatPayload::default())?;

        Ok(records)
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
    pub async fn exec(self, conn: &DbConn) -> Result<(Receiver<StreamContent>, Id, Id)> {
        let chat_service = ChatService::new(conn.clone());

        let (sender, receiver) = mpsc::channel::<StreamContent>(20);
        let (message_id, reply_id, _) = chat_service
            .send_message(
                SendMessagePayload {
                    chat_id: self.chat_id,
                    message: self.message,
                },
                sender,
            )
            .await?;
        Ok((receiver, message_id, reply_id))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendMessageCommand {
    pub message_id: Id,
}

impl ResendMessageCommand {
    pub async fn exec(self, conn: &DbConn) -> Result<(Receiver<StreamContent>, Id, Id)> {
        let chat_service = ChatService::new(conn.clone());

        let (sender, receiver) = mpsc::channel::<StreamContent>(20);
        let (message_id, reply_id, _) = chat_service
            .resend_message(
                ResendMessagePayload {
                    id: self.message_id,
                },
                sender,
            )
            .await?;
        Ok((receiver, message_id, reply_id))
    }
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
    payload: UpdatePromptPayload,
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
    pub fn exec(self, conn: &DbConn) -> Result<Id> {
        let prompt_market_service = PromptMarketService::new(conn.clone());

        let chat_id = prompt_market_service.install_market_prompt_and_create_chat(
            InstallMarketPromptPayload {
                prompt: MarketPrompt {
                    name: self.name,
                    content: self.content,
                },
                user_id: Id::local(),
            },
        )?;

        Ok(chat_id)
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
