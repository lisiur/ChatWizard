use crate::result::Result;
use crate::OpenAIApi;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListModelResponseData {
    data: Vec<Model>,
}

impl OpenAIApi {
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let models = self
            .client
            .get_json::<ListModelResponseData>("https://api.openai.com/v1/models")
            .await?;

        Ok(models.data)
    }
}
