use std::fmt::Display;

use tiktoken_rs::{self, cl100k_base, model::get_context_size};

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug)]
pub struct OpenAIChatParams {
    /// ID of the model to use.
    pub model: String,

    /// The messages to generate chat completions for, in the chat format.
    pub messages: Vec<OpenAIChatMessage>,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    /// Tokens will be sent as data-only server-sent events as they become available,
    /// with the stream terminated by a data: [DONE] message.
    pub stream: bool,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// The maximum number of tokens to generate in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl OpenAIChatParams {
    pub fn calc_tokens(&self) -> usize {
        let context_size = get_context_size(&self.model);
        let mut tokens = 0;
        for message in &self.messages {
            tokens += message.tokens();
        }

        context_size.saturating_sub(tokens)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OpenAIChatMessage {
    pub role: OpenAIChatRole,
    pub content: String,
}

impl OpenAIChatMessage {
    pub fn tokens(&self) -> usize {
        Self::calc_tokens(&self.role, &self.content)
    }

    pub fn calc_tokens(role: &OpenAIChatRole, content: &str) -> usize {
        let bpe = cl100k_base().unwrap();

        let mut num_tokens = 0;
        num_tokens += 4; // every message follows <im_start>{role/name}\n{content}<im_end>\n
        num_tokens += bpe.encode_with_special_tokens(&role.to_string()).len();
        num_tokens += bpe.encode_with_special_tokens(content).len();

        num_tokens
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OpenAIChatRole {
    System,
    User,
    Assistant,
}

impl Display for OpenAIChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAIChatRole::System => write!(f, "system"),
            OpenAIChatRole::User => write!(f, "user"),
            OpenAIChatRole::Assistant => write!(f, "assistant"),
        }
    }
}
