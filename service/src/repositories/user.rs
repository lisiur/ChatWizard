use crate::models::user::{NewUser, PatchUser};
use crate::result::Result;
use crate::{conn::DbConn, models::user::User, types::Id};
use diesel::*;

pub struct UserRepo(DbConn);

impl UserRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn count(&self) -> Result<i64> {
        use crate::schema::users::dsl::*;

        users
            .count()
            .get_result(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select_by_id(&self, user_id: Id) -> Result<User> {
        use crate::schema::users::dsl::*;

        users
            .filter(id.eq(user_id))
            .first::<User>(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn insert_if_not_exist(&self, user: &NewUser) -> Result<usize> {
        use crate::schema::users::dsl::*;

        let size = diesel::insert_into(users)
            .values(user)
            .on_conflict(id)
            .do_nothing()
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn insert(&self, user: &NewUser) -> Result<usize> {
        use crate::schema::users::dsl::*;

        let size = diesel::insert_into(users)
            .values(user)
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn update(&self, patch: &PatchUser) -> Result<usize> {
        use crate::schema::users::dsl::*;

        let size = diesel::update(users)
            .filter(id.eq(patch.id))
            .set(patch)
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn delete_by_id(&self, user_id: Id) -> Result<usize> {
        use crate::schema::users::dsl::*;

        let size = diesel::delete(users)
            .filter(id.eq(user_id))
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }
}

#[cfg(test)]
mod test {
    use diesel::{Connection, SqliteConnection};
    use once_cell::sync::OnceCell;
    use uuid::Uuid;

    use crate::{
        conn::DbConn, models::user::NewUser, result::Result, test::establish_connection, types::Id,
    };

    use super::UserRepo;

    static USER_REPO: OnceCell<UserRepo> = OnceCell::new();

    fn create_user_repo() -> &'static UserRepo {
        USER_REPO.get_or_init(|| {
            let conn = establish_connection();
            UserRepo::new(conn)
        })
    }

    fn new_user() -> NewUser {
        NewUser {
            id: Id::random(),
            name: "test".to_string(),
            email: "test@email.com".to_string(),
            password: "test".to_string(),
        }
    }

    #[test]
    fn test_create_user() {
        let user_repo = create_user_repo();

        let user = new_user();
        let size = user_repo.insert(&user).unwrap();

        assert_eq!(size, 1);

        user_repo.delete_by_id(user.id).unwrap();
    }

    #[test]
    fn test_query_user() {
        let user_repo = create_user_repo();

        let user = new_user();

        let user_id = user.id;

        user_repo.insert(&user).unwrap();

        let user2 = user_repo.select_by_id(user_id).unwrap();

        assert_eq!(user_id, user2.id);

        user_repo.delete_by_id(user_id).unwrap();
    }

    #[test]
    fn test_update_user() {
        let user_repo = create_user_repo();

        let user = new_user();
        let user_id = user.id;

        user_repo.insert(&user).unwrap();
        assert!(user_repo.select_by_id(user_id).is_ok());

        user_repo.delete_by_id(user_id).unwrap();
        assert!(user_repo.select_by_id(user_id).is_err());
    }
}
