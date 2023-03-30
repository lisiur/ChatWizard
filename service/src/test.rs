use once_cell::sync::OnceCell;

use crate::{
    database::DbConn, init, models::setting::PatchSetting, repositories::setting::SettingRepo,
    types::Id,
};

static DB_CONN: OnceCell<DbConn> = OnceCell::new();

pub fn establish_connection() -> DbConn {
    DB_CONN
        .get_or_init(|| {
            dotenvy::dotenv().unwrap();
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let api_key = std::env::var("API_KEY").unwrap();
            let proxy = std::env::var("PROXY").ok();

            let conn = init(&database_url).unwrap();

            let setting_repo = SettingRepo::new(conn.clone());

            setting_repo
                .update(&PatchSetting {
                    user_id: Id::local(),
                    api_key: Some(api_key),
                    proxy,
                    ..Default::default()
                })
                .unwrap();

            conn
        })
        .clone()
}
