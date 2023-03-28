use diesel::{Connection, SqliteConnection};
use once_cell::sync::OnceCell;
use uuid::Uuid;

use crate::{conn::DbConn, init, models::user::NewUser, result::Result, types::Id};

static DB_CONN: OnceCell<DbConn> = OnceCell::new();

pub fn establish_connection() -> DbConn {
    DB_CONN
        .get_or_init(|| {
            dotenvy::dotenv().unwrap();
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let conn = DbConn::new(&database_url);

            init(conn.clone()).unwrap();

            conn
        })
        .clone()
}
