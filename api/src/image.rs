use crate::result::Result;
use crate::OpenAIApi;

#[derive(serde::Serialize, Default, Debug)]
pub struct CreateImageRequestParams {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateImageResponseData {
    pub created: Option<u32>,
    pub data: Vec<CreateImageResponseDataItem>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateImageResponseDataItem {
    pub url: String,
}

impl OpenAIApi {
    pub async fn create_image(
        &self,
        params: CreateImageRequestParams,
    ) -> Result<CreateImageResponseData> {
        let data = self
            .client
            .post_json::<CreateImageResponseData>(
                "https://api.openai.com/v1/images/generations",
                params,
            )
            .await?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_image_basic() {
        dotenv::dotenv().unwrap();

        let api = OpenAIApi::new(&std::env::var("OPENAI_API").unwrap());
        api.set_proxy(&std::env::var("PROXY").unwrap()).await;

        let data = api
            .create_image(CreateImageRequestParams {
                prompt: "a lovely dog".to_string(),
                ..CreateImageRequestParams::default()
            })
            .await
            .unwrap();

        let urls = data
            .data
            .into_iter()
            .map(|item| item.url)
            .collect::<Vec<String>>();

        assert!(!urls.is_empty());
    }
}
