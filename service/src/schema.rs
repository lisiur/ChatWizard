// @generated automatically by Diesel CLI.

diesel::table! {
    chat_logs (id) {
        id -> Binary,
        chat_id -> Binary,
        role -> Text,
        message -> Text,
        model -> Text,
        tokens -> Integer,
        cost -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chat_models (id) {
        id -> Binary,
        name -> Text,
        description -> Text,
        price -> Float,
        unit -> Text,
        vendor -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        id -> Binary,
        user_id -> Binary,
        title -> Text,
        prompt_id -> Nullable<Binary>,
        config -> Text,
        cost -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    prompt_sources (id) {
        id -> Binary,
        name -> Text,
        description -> Text,
        url -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    prompts (id) {
        id -> Binary,
        name -> Text,
        content -> Text,
        user_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    settings (id) {
        id -> Binary,
        user_id -> Binary,
        language -> Text,
        theme -> Text,
        api_key -> Nullable<Text>,
        proxy -> Nullable<Text>,
        forward_url -> Nullable<Text>,
        forward_api_key -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Binary,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chat_logs,
    chat_models,
    chats,
    prompt_sources,
    prompts,
    settings,
    users,
);
