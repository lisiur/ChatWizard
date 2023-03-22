use std::sync::Arc;

use futures::lock::Mutex;

use crate::chat::chat_manager::ChatManager;
use crate::project::Project;
use crate::prompt::prompt_manager::PromptManager;
use crate::result::Result;
use crate::setting::Setting;
pub struct AppState {
    pub setting: Arc<Mutex<Setting>>,
    pub chat_manager: Arc<Mutex<ChatManager>>,
    pub prompt_manager: Arc<Mutex<PromptManager>>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        // Init setting
        let setting = Setting::init(&Project::setting_path()).await?;

        // Init prompt manager
        let prompt_manager = PromptManager::init().await?;
        let prompt_manager = Arc::new(Mutex::new(prompt_manager));

        // Init chat manager
        let chat_manager = ChatManager::init().await?;

        let state = AppState {
            setting: Arc::new(Mutex::new(setting)),
            chat_manager: Arc::new(Mutex::new(chat_manager)),
            prompt_manager,
        };

        Ok(state)
    }
}
