use crate::result::Result;

use crate::OpenAIApi;

#[derive(serde::Serialize, Debug)]
pub struct CreateCompletionRequestParams {
    model: String,
    prompt: String,
    suffix: Option<String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<u32>,
}

impl Default for CreateCompletionRequestParams {
    fn default() -> Self {
        Self {
            model: "text-davinci-003".to_string(),
            prompt: "".to_string(),
            suffix: None,
            temperature: Some(0.0),
            top_p: None,
            max_tokens: Some(1500),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateCompletionResponseData {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CreateCompletionResponseChoices>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateCompletionResponseChoices {
    pub text: Option<String>,
    pub index: Option<usize>,
    pub logprobs: Option<CreateCompletionResponseChoicesLogprobs>,
    pub finish_reason: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateCompletionResponseChoicesLogprobs {
    pub tokens: Option<Vec<String>>,
    pub token_logprobs: Option<Vec<f32>>,
    pub text_offset: Option<usize>,
}

impl OpenAIApi {
    pub async fn create_completion(
        &self,
        params: CreateCompletionRequestParams,
    ) -> Result<CreateCompletionResponseData> {
        self.client
            .post_json::<CreateCompletionResponseData>(
                "https://api.openai.com/v1/completions",
                params,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        dotenv::dotenv().unwrap();

        let mut api = OpenAIApi::new(std::env::var("OPENAI_API").ok().as_deref());
        api.set_proxy(&std::env::var("PROXY").unwrap());

        let data = api
            .create_completion(CreateCompletionRequestParams {
                prompt: "say Hello! to me".to_string(),
                ..CreateCompletionRequestParams::default()
            })
            .await
            .unwrap();

        assert_eq!(
            data.choices
                .into_iter()
                .next()
                .and_then(|choice| choice.text),
            Some("\n\nHello!".to_string())
        );
    }
}
