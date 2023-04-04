use std::sync::{Arc, Mutex, MutexGuard};

use diesel::{Connection, SqliteConnection};

#[derive(Clone)]
pub struct DbConn(pub Arc<Mutex<diesel::sqlite::SqliteConnection>>);

impl DbConn {
    pub fn new(db_url: &str) -> Self {
        let conn = SqliteConnection::establish(db_url).unwrap();
        Self(Arc::new(Mutex::new(conn)))
    }

    pub fn clone_self(&self) -> Self {
        Self(self.0.clone())
    }

    pub fn conn(&self) -> MutexGuard<SqliteConnection> {
        self.0.lock().unwrap()
    }
}
