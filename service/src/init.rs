use crate::error::Error;
use crate::models::chat_model::NewChatModel;
use crate::models::prompt_source::NewPromptSource;
use crate::models::setting::{NewSetting, Theme};
use crate::repositories::chat_model::ChatModelRepo;
use crate::repositories::prompt_source::PromptSourceRepo;
use crate::repositories::setting::SettingRepo;
use crate::result::Result;
use crate::{database::DbConn, models::user::NewUser, repositories::user::UserRepo, types::Id};
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| Error::Unknown(err.to_string()))?;

    Ok(())
}

pub fn init(db_url: &str) -> Result<DbConn> {
    let conn = DbConn::new(db_url);

    run_migrations(&mut *conn.conn())?;

    // Create local user
    let user_repo = UserRepo::new(conn.clone());
    let local_user = NewUser {
        id: Id::local(),
        name: "local".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };
    user_repo.insert_if_not_exist(&local_user)?;

    // Create local setting
    let setting_repo = SettingRepo::new(conn.clone());
    let local_setting = NewSetting {
        id: Id::local(),
        user_id: Id::local(),
        language: "enUS".to_string(),
        theme: Theme::System.into(),
        api_key: None,
        proxy: None,
        forward_url: None,
        forward_api_key: false,
    };
    setting_repo.insert_if_not_exist(&local_setting)?;

    // Create chat models
    let chat_models = vec![
        NewChatModel {
            id: Id::from("e8621eb4-fee8-42a6-9627-f34539881aa8"),
            name: "gpt-3.5-turbo".to_string(),
            description: "".to_string(),
            price: 0.002,
            unit: "USD".to_string(),
            vendor: "openai".to_string(),
        },
        NewChatModel {
            id: Id::from("a5224f79-6d95-439e-a312-22cce02fd61f"),
            name: "gpt-4".to_string(),
            description: "".to_string(),
            price: 0.06,
            unit: "USD".to_string(),
            vendor: "openai".to_string(),
        },
    ];
    let chat_model_repo = ChatModelRepo::new(conn.clone());
    for chat_model in chat_models {
        chat_model_repo.insert_or_update(&chat_model)?;
    }

    // Create prompt sources
    let prompt_sources = vec![
        NewPromptSource {
            id: Id::from("b46134f1-26d4-4644-aba5-02b2796f088a"),
            name: "awesome-chatgpt-prompts".to_string(),
            description: "https://github.com/f/awesome-chatgpt-prompts".to_string(),
            url: "https://raw.githubusercontent.com/f/awesome-chatgpt-prompts/main/prompts.csv".to_string(),
            r#type: "csv".to_string(),
        },
        NewPromptSource {
            id: Id::from("7af1f226-5a10-492d-b466-8eff82931b57"),
            name: "awesome-chatgpt-prompts-zh".to_string(),
            description: "https://github.com/PlexPt/awesome-chatgpt-prompts-zh".to_string(),
            url: "https://raw.githubusercontent.com/PlexPt/awesome-chatgpt-prompts-zh/main/prompts-zh.json".to_string(),
            r#type: "json".to_string(),
        }
    ];
    let prompt_source_repo = PromptSourceRepo::new(conn.clone());
    for prompt_source in prompt_sources {
        prompt_source_repo.insert_or_update(&prompt_source)?;
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use crate::{
        repositories::{setting::SettingRepo, user::UserRepo},
        test::establish_connection,
        types::Id,
    };

    #[test]
    fn test_init() {
        let conn = establish_connection();

        let user_repo = UserRepo::new(conn.clone());
        let setting_repo = SettingRepo::new(conn);

        let local_user = user_repo.select_by_id(Id::local());
        let local_setting = setting_repo.select_by_user_id(Id::local());

        assert!(local_user.is_ok());
        assert!(local_setting.is_ok());
    }
}
