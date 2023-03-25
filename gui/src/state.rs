use std::sync::Arc;

use futures::lock::Mutex;

use crate::chat::chat_manager::ChatManager;
use crate::market_prompt::market_prompt_manager::MarketPromptManager;
use crate::project::Project;
use crate::prompt::prompt_manager::PromptManager;
use crate::result::Result;
use crate::service::prompt_market_service::PromptMarketService;
use crate::setting::Setting;
pub struct AppState {
    pub setting: Arc<Mutex<Setting>>,
    pub chat_manager: Arc<Mutex<ChatManager>>,
    pub prompt_manager: Arc<Mutex<PromptManager>>,
    pub prompt_market_service: Arc<Mutex<PromptMarketService>>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        // Init setting
        let setting = Arc::new(Mutex::new(
            Setting::init(&Project::default().setting_path()).await?,
        ));

        // Init prompt manager
        let prompt_manager = PromptManager::init().await?;
        let prompt_manager = Arc::new(Mutex::new(prompt_manager));

        let market_prompt_manager = Arc::new(Mutex::new(MarketPromptManager::init().await?));

        // Init chat manager
        let chat_manager = ChatManager::init().await?;

        let state = AppState {
            setting: setting.clone(),
            chat_manager: Arc::new(Mutex::new(chat_manager)),
            prompt_manager: prompt_manager.clone(),
            prompt_market_service: Arc::new(Mutex::new(PromptMarketService::new(
                market_prompt_manager.clone(),
                prompt_manager.clone(),
                setting.clone(),
            ))),
        };

        Ok(state)
    }
}
