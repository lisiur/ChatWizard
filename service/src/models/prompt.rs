use chrono::NaiveDateTime;

use crate::types::Id;

struct Prompt {
    id: Id,
    name: String,
    content: String,
    user_id: Id,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
