use std::sync::{Arc, Mutex, MutexGuard};

use diesel::{Connection, SqliteConnection};

#[derive(Clone)]
pub struct DbConn(pub Arc<Mutex<diesel::sqlite::SqliteConnection>>);

impl DbConn {
    pub fn new(db_url: &str) -> Self {
        dbg!(db_url);
        let conn = SqliteConnection::establish(db_url).unwrap();
        Self(Arc::new(Mutex::new(conn)))
    }

    pub fn conn(&self) -> MutexGuard<SqliteConnection> {
        self.0.lock().unwrap()
    }
}
