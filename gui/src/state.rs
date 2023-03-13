use std::sync::Arc;

use askai_api::Topic;
use futures::lock::Mutex;

use crate::{setting::Setting, utils::create_topic};

pub struct AppState {
    pub setting: Mutex<Setting>,
    pub topic: Arc<Mutex<Topic>>,
}

impl AppState {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let setting = Setting::init()?;
        let state = AppState {
            topic: Arc::new(Mutex::new(create_topic(&setting).await)),
            setting: Mutex::new(setting),
        };

        Ok(state)
    }
}
