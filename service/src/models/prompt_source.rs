use chrono::NaiveDateTime;

pub struct PromptSource {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
