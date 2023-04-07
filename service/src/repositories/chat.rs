use crate::models::chat::{NewChat, PatchChat};
use crate::result::Result;
use crate::schema::chats;
use crate::{database::DbConn, models::chat::Chat, types::Id};
use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::QueryDsl;

#[derive(Clone)]
pub struct ChatRepo(DbConn);

impl ChatRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn count(&self, user_id: Id) -> Result<i64> {
        chats::table
            .filter(chats::user_id.eq(user_id))
            .count()
            .get_result(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select_casual(&self, user_id: Id) -> Result<Chat> {
        chats::table
            .as_query()
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.eq(chats::user_id))
            .first::<Chat>(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select_all_except_casual(&self, user_id: Id) -> Result<Vec<Chat>> {
        let stick_chats = self.select_stick(user_id)?;
        let nonstick_chats = self.select_non_stick(user_id)?;
        let archived = self.select_archived(user_id)?;

        let all_chats = stick_chats.into_iter().chain(nonstick_chats).chain(archived).collect();

        Ok(all_chats)
    }

    pub fn select_non_stick(&self, user_id: Id) -> Result<Vec<Chat>> {
        chats::table
            .as_query()
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(false))
            .order(chats::sort.asc())
            .load::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_stick(&self, user_id: Id) -> Result<Vec<Chat>> {
        chats::table
            .as_query()
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(true))
            .order(chats::sort.asc())
            .load::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_archived(&self, user_id: Id) -> Result<Vec<Chat>> {
        chats::table
            .as_query()
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(true))
            .order(chats::archived_at.desc())
            .load::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_non_stick_min_order(&self, user_id: Id) -> Result<i32> {
        match chats::table
            .select(chats::sort)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(false))
            .order(chats::sort.asc())
            .first::<i32>(&mut *self.0.conn())
        {
            Ok(order) => Ok(order),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(0),
                _ => Err(err.into()),
            },
        }
    }

    pub fn select_non_stick_max_order(&self, user_id: Id) -> Result<i32> {
        match chats::table
            .select(chats::sort)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(false))
            .order(chats::sort.desc())
            .first::<i32>(&mut *self.0.conn())
        {
            Ok(order) => Ok(order),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(0),
                _ => Err(err.into()),
            },
        }
    }

    pub fn select_stick_min_order(&self, user_id: Id) -> Result<i32> {
        match chats::table
            .select(chats::sort)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(true))
            .order(chats::sort.asc())
            .first::<i32>(&mut *self.0.conn())
        {
            Ok(order) => Ok(order),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(0),
                _ => Err(err.into()),
            },
        }
    }

    pub fn decrease_stick_order(&self, user_id: Id, from: i32, to: i32) -> Result<usize> {
        diesel::update(chats::table)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(true))
            .filter(chats::sort.ge(from))
            .filter(chats::sort.le(to))
            .set(chats::sort.eq(chats::sort - 1))
            .execute(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn increase_stick_order(&self, user_id: Id, from: i32, to: i32) -> Result<usize> {
        diesel::update(chats::table)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(true))
            .filter(chats::sort.ge(from))
            .filter(chats::sort.le(to))
            .set(chats::sort.eq(chats::sort + 1))
            .execute(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn decrease_non_stick_order(&self, user_id: Id, from: i32, to: i32) -> Result<usize> {
        diesel::update(chats::table)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(false))
            .filter(chats::sort.ge(from))
            .filter(chats::sort.le(to))
            .set(chats::sort.eq(chats::sort - 1))
            .execute(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn increase_non_stick_order(&self, user_id: Id, from: i32, to: i32) -> Result<usize> {
        log::debug!(
            "increase_non_stick_order: user: {}, from: {}, to: {}",
            user_id,
            from,
            to
        );
        diesel::update(chats::table)
            .filter(chats::user_id.eq(user_id))
            .filter(chats::id.ne(user_id))
            .filter(chats::archive.eq(false))
            .filter(chats::stick.eq(false))
            .filter(chats::sort.ge(from))
            .filter(chats::sort.le(to))
            .set(chats::sort.eq(chats::sort + 1))
            .execute(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_by_id(&self, id: Id) -> Result<Chat> {
        chats::table
            .filter(chats::id.eq(id))
            .first::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_by_user_id(&self, user_id: Id) -> Result<Vec<Chat>> {
        chats::table
            .filter(chats::user_id.eq(user_id))
            .load::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn insert(&self, chat: &NewChat) -> Result<usize> {
        let size = diesel::insert_into(chats::table)
            .values(chat)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn insert_if_not_exist(&self, chat: &NewChat) -> Result<usize> {

        let size = diesel::insert_into(chats::table)
            .values(chat)
            .on_conflict(chats::id)
            .do_nothing()
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn update(&self, chat: &PatchChat) -> Result<usize> {
        log::debug!("update chat: {:?}", chat);
        let size = diesel::update(chats::table)
            .filter(chats::id.eq(chat.id))
            .set(chat)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn add_cost_and_update(&self, id: Id, cost: f32) -> Result<usize> {
        let size = diesel::update(chats::table)
            .filter(chats::id.eq(id))
            .set(chats::cost.eq(chats::cost + cost))
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn delete_by_id(&self, id: Id) -> Result<usize> {
        diesel::delete(chats::table.filter(chats::id.eq(id)))
            .execute(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn update_deleted_prompt(&self, prompt_id: Id) -> Result<usize> {
        let size = diesel::update(chats::table)
            .filter(chats::prompt_id.eq(prompt_id))
            .set(chats::prompt_id.eq(None::<Id>))
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::OnceCell;

    use crate::{
        models::chat::{NewChat, PatchChat},
        test::establish_connection,
        types::Id,
    };

    use super::ChatRepo;

    static CHAT_REPO: OnceCell<ChatRepo> = OnceCell::new();

    fn create_chat_repo() -> &'static ChatRepo {
        CHAT_REPO.get_or_init(|| {
            let conn = establish_connection();
            ChatRepo::new(conn)
        })
    }

    fn new_chat(title: &str) -> NewChat {
        NewChat {
            id: Id::random(),
            user_id: Id::local(),
            title: title.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_create_chat() {
        let repo = create_chat_repo();

        let chat = new_chat("test");
        let id = chat.id;

        repo.insert(&chat).unwrap();

        assert!(repo.select_by_id(id).is_ok());

        repo.delete_by_id(id).unwrap();
    }

    #[test]
    fn test_select_chats() {
        let repo = create_chat_repo();

        assert!(repo.select_by_user_id(Id::local()).is_ok());
    }

    #[test]
    fn update_chat() {
        let repo = create_chat_repo();
        let chat = new_chat("test");
        repo.insert(&chat).unwrap();

        let patch_chat = PatchChat {
            id: chat.id,
            title: Some("test2".to_string()),
            ..Default::default()
        };

        repo.update(&patch_chat).unwrap();

        let chat = repo.select_by_id(chat.id).unwrap();
        assert_eq!(chat.title, "test2");

        repo.delete_by_id(chat.id).unwrap();
    }
}
