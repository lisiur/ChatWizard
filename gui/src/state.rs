use std::sync::Arc;

use futures::lock::Mutex;

use crate::chat::ChatManager;
use crate::project::Project;
use crate::prompt::PromptManager;
use crate::result::Result;
use crate::setting::Setting;
pub struct AppState {
    pub setting: Arc<Mutex<Setting>>,
    pub chat_manager: Arc<Mutex<ChatManager>>,
    pub prompt_manager: Arc<Mutex<PromptManager>>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let project = Project::new();

        // Init setting
        let setting = Setting::init(&project.setting_path).await?;

        // Init prompt manager
        let prompt_manager =
            PromptManager::init(&project.prompt_metadata_path, &project.prompt_data_dir).await?;
        let prompt_manager = Arc::new(Mutex::new(prompt_manager));

        // Init chat manager
        let chat_manager = ChatManager::init(
            &project.chat_metadata_path,
            &project.chat_data_dir,
            prompt_manager.clone(),
        )
        .await?;

        let state = AppState {
            setting: Arc::new(Mutex::new(setting)),
            chat_manager: Arc::new(Mutex::new(chat_manager)),
            prompt_manager,
        };

        Ok(state)
    }
}
