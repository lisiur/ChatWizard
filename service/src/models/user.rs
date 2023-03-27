use chrono::NaiveDateTime;
use diesel::{
    AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
};

use crate::conn::DbConn;
use crate::result::Result;
use crate::schema::users;
use crate::types::Id;

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    id: Id,
    name: String,
    email: String,
    password: String,
}

impl NewUser {
    fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: Id::random(),
            name,
            email,
            password,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
struct PatchUser {
    id: Id,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Queryable, Selectable, Debug, Clone, PartialEq, Eq)]
struct User {
    id: Id,
    name: String,
    email: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl User {
    pub fn with_id(id: Id) {
        use crate::schema::users::dsl::*;
        todo!()
    }
}

struct UserRepo(DbConn);

impl UserRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select(&self, user_id: Id) -> Result<User> {
        use crate::schema::users::dsl::*;
        users
            .filter(id.eq(user_id))
            .first::<User>(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn insert(&self, user: &NewUser) -> Result<usize> {
        use crate::schema::users::dsl::*;
        let mut conn = self.0.conn();
        let size = diesel::insert_into(users)
            .values(user)
            .execute(&mut *conn)?;

        Ok(size)
    }

    pub fn update(&self, patch: &PatchUser) -> Result<usize> {
        use crate::schema::users::dsl::*;
        let mut conn = self.0.conn();
        let size = diesel::update(users)
            .filter(id.eq(patch.id))
            .set(patch)
            .execute(&mut *conn)?;

        Ok(size)
    }
}

#[cfg(test)]
mod test {
    use std::ptr::eq;

    use diesel::{Connection, SqliteConnection};

    use crate::{conn::DbConn, result::Result};

    fn establish_connection() -> DbConn {
        dotenvy::dotenv().unwrap();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DbConn::new(&database_url)
    }

    #[test]
    fn test_create_user() {
        let conn = establish_connection();

        let user_repo = super::UserRepo::new(conn);

        let user = super::NewUser::new(
            "test".to_string(),
            "test@email.com".to_string(),
            "test".to_string(),
        );

        let size = user_repo.insert(&user).unwrap();

        assert_eq!(size, 1);
    }

    #[test]
    fn test_query_user() {
        let conn = establish_connection();

        let user_repo = super::UserRepo::new(conn);

        let user = super::NewUser::new(
            "test".to_string(),
            "test@email.com".to_string(),
            "test".to_string(),
        );

        let user_id = user.id;

        user_repo.insert(&user).unwrap();

        let user2 = user_repo.select(user_id).unwrap();

        assert_eq!(user_id, user2.id);
    }

    #[test]
    fn test_update_user() {}
}
