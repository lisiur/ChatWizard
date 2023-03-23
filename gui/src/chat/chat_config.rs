use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatConfig {
    pub model: String,

    pub prompt_id: Option<Uuid>,

    pub max_backtrack: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            prompt_id: None,
            max_backtrack: 2,
            temperature: Some(1.0),
            top_p: None,
            n: Some(1),
            stop: None,
            max_tokens: None,
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            user: None,
        }
    }
}
