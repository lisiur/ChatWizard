use serde::Serialize;

use crate::{
    models::prompt_source::PromptSource,
    repositories::{
        chat::ChatRepo, prompt::PromptRepo, prompt_source::PromptSourceRepo, setting::SettingRepo,
    },
    result::Result,
    ChatConfig, DbConn, Id, NewChat, NewPrompt,
};

pub struct PromptMarketService {
    #[allow(unused)]
    conn: DbConn,
    chat_repo: ChatRepo,
    setting_repo: SettingRepo,
    prompt_repo: PromptRepo,
    prompt_source_repo: PromptSourceRepo,
}

impl PromptMarketService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            chat_repo: ChatRepo::new(conn.clone()),
            setting_repo: SettingRepo::new(conn.clone()),
            prompt_repo: PromptRepo::new(conn.clone()),
            prompt_source_repo: PromptSourceRepo::new(conn.clone()),
            conn,
        }
    }

    pub fn get_prompt_sources(&self) -> Result<Vec<PromptSource>> {
        let prompt_sources = self.prompt_source_repo.select()?;
        Ok(prompt_sources)
    }

    pub async fn get_prompt_source_prompts(
        &self,
        prompt_source_id: Id,
    ) -> Result<Vec<MarketPrompt>> {
        let prompt_source = self.prompt_source_repo.select_by_id(prompt_source_id)?;

        let PromptSource { url, r#type, .. } = prompt_source;

        let setting = self.setting_repo.select_by_user_id(Id::local())?;
        let client = setting.create_client();

        let market_prompts = match r#type.as_str() {
            "json" => {
                let res = client.get(&url).await?;
                let prompts: Vec<serde_json::Value> = res.json().await?;
                prompts
                    .into_iter()
                    .map(|p| MarketPrompt {
                        name: p["act"].as_str().unwrap_or("").to_string(),
                        content: p["prompt"].as_str().unwrap_or("").to_string(),
                    })
                    .collect::<Vec<MarketPrompt>>()
            }
            "csv" => {
                let res = client.get(&url).await?;
                let body = res.text().await?;

                let mut rdr = csv::Reader::from_reader(body.as_bytes());

                let mut prompts = vec![];
                for result in rdr.records() {
                    let record = result?;
                    let name = record[0].to_string();
                    let content = record[1].to_string();
                    let market_prompt = MarketPrompt { name, content };
                    prompts.push(market_prompt);
                }
                prompts
            }
            _ => vec![],
        };

        Ok(market_prompts)
    }

    pub fn install_market_prompt(&self, payload: InstallMarketPromptPayload) -> Result<Id> {
        let id = Id::random();
        let prompt = NewPrompt {
            id,
            name: payload.prompt.name,
            content: payload.prompt.content,
            user_id: payload.user_id,
        };
        self.prompt_repo.insert(&prompt)?;

        Ok(id)
    }

    pub fn install_market_prompt_and_create_chat(
        &self,
        payload: InstallMarketPromptPayload,
    ) -> Result<Id> {
        let name = payload.prompt.name.clone();
        let user_id = payload.user_id;
        let prompt_id = self.install_market_prompt(payload)?;

        let min_sort = self.chat_repo.select_non_stick_min_order(user_id)?;
        let chat_id = Id::random();
        let chat = NewChat {
            id: chat_id,
            prompt_id: Some(prompt_id),
            user_id,
            title: name,
            config: ChatConfig::default().into(),
            cost: 0.0,
            vendor: "openai".to_string(),
            sort: min_sort - 1,
            stick: false,
        };
        self.chat_repo.insert(&chat)?;
        Ok(chat_id)
    }
}

#[derive(Serialize)]
pub struct MarketPrompt {
    pub name: String,
    pub content: String,
}

pub struct InstallMarketPromptPayload {
    pub prompt: MarketPrompt,
    pub user_id: Id,
}
