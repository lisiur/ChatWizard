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
pub struct NewUser {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct PatchUser {
    pub id: Id,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Queryable, Selectable, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn with_id(id: Id) {
        use crate::schema::users::dsl::*;
        todo!()
    }
}
