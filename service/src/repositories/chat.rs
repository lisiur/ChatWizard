use crate::database::pagination::*;
use crate::models::chat::{NewChat, PatchChat};
use crate::result::Result;
use crate::schema::chats;
use crate::types::PageQueryParams;
use crate::{database::DbConn, models::chat::Chat, types::Id};
use diesel::query_builder::AsQuery;
use diesel::QueryDsl;
use diesel::*;

pub struct ChatRepo(DbConn);

impl ChatRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn count(&self) -> Result<i64> {
        use crate::schema::chats::dsl::*;

        chats
            .count()
            .get_result(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select(&self, params: PageQueryParams) -> Result<PaginatedRecords<Chat>> {
        use crate::schema::chats::dsl::*;

        let result = chats
            .as_query()
            .paginate(params.page)
            .per_page(params.per_page)
            .load_and_count_pages::<Chat>(&mut self.0.conn())?;

        Ok(result)
    }

    pub fn select_by_id(&self, chat_id: Id) -> Result<Chat> {
        use crate::schema::chats::dsl::*;

        chats
            .filter(id.eq(chat_id))
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
        use crate::schema::chats::dsl::*;

        let size = diesel::insert_into(chats)
            .values(chat)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn update(&self, chat: &PatchChat) -> Result<usize> {
        use crate::schema::chats::dsl::*;

        let size = diesel::update(chats)
            .filter(id.eq(chat.id))
            .set(chat)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn delete_by_id(&self, chat_id: Id) -> Result<usize> {
        use crate::schema::chats::dsl::*;

        diesel::delete(chats.filter(id.eq(chat_id)))
            .execute(&mut *self.0.conn())
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::OnceCell;

    use crate::{
        models::chat::{ChatConfig, NewChat, PatchChat},
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
            prompt_id: None,
            config: ChatConfig::default().into(),
            cost: 0.0,
            vendor: "openai".to_string(),
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
            config: None,
            cost: None,
            prompt_id: None,
            vendor: None,
        };

        repo.update(&patch_chat).unwrap();

        let chat = repo.select_by_id(chat.id).unwrap();
        assert_eq!(chat.title, "test2");

        repo.delete_by_id(chat.id).unwrap();
    }
}
