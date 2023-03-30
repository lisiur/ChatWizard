use diesel::backend::RawValue;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::{Binary, Text};
use diesel::sqlite::Sqlite;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{self, Uuid};

use crate::error::StreamError;

#[derive(
    Debug,
    Clone,
    Copy,
    FromSqlRow,
    AsExpression,
    Hash,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[diesel(sql_type = Binary)]
pub struct Id(pub uuid::Uuid);

impl Default for Id {
    fn default() -> Self {
        Self::local()
    }
}

impl Id {
    pub fn local() -> Self {
        Self(uuid::Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap())
    }

    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<Id> for uuid::Uuid {
    fn from(s: Id) -> Self {
        s.0
    }
}

impl From<uuid::Uuid> for Id {
    fn from(s: uuid::Uuid) -> Self {
        Self(s)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(uuid::Uuid::parse_str(s).unwrap())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql<Binary, Sqlite> for Id {
    fn from_sql(bytes: RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        Ok(Self(Uuid::from_slice(&bytes).unwrap()))
    }
}

impl ToSql<Binary, Sqlite> for Id {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        <[u8] as ToSql<Binary, Sqlite>>::to_sql(self.0.as_bytes(), out)
    }
}

// TextWrapper

#[derive(AsExpression, FromSqlRow, Serialize, Deserialize, Debug)]
#[diesel(sql_type = Text)]
pub struct TextWrapper<T>(pub T);

impl<T> AsRef<T> for TextWrapper<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for TextWrapper<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: FromStr> FromSql<Text, Sqlite> for TextWrapper<T> {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        let string = String::from_utf8(bytes)?;
        let inner = T::from_str(&string).map_err(|_err| "error")?;
        Ok(TextWrapper(inner))
    }
}

impl<T: AsRef<str> + fmt::Debug> ToSql<Text, Sqlite> for TextWrapper<T> {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        <str as ToSql<Text, Sqlite>>::to_sql(self.0.as_ref(), out)
    }
}

// Serde Wrapper

#[derive(AsExpression, FromSqlRow, serde::Serialize, serde::Deserialize, Debug)]
#[diesel(sql_type = Text)]
pub struct JsonWrapper<T>(pub T);

impl<T> AsRef<T> for JsonWrapper<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for JsonWrapper<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: serde::de::DeserializeOwned + fmt::Debug> FromSql<Text, Sqlite> for JsonWrapper<T> {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        let inner = serde_json::from_slice::<T>(&bytes)?;
        Ok(JsonWrapper(inner))
    }
}

impl<T: serde::Serialize + fmt::Debug> ToSql<Text, Sqlite> for JsonWrapper<T> {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(serde_json::to_string(&self.0).unwrap());

        Ok(IsNull::No)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct PageQueryParams<T, U> {
    pub page: i64,
    pub per_page: i64,
    pub user_id: Id,
    pub query: T,
    pub sort: U,
}

impl<T: Default, U: Default> Default for PageQueryParams<T, U> {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: i64::MAX,
            user_id: Id::local(),
            query: Default::default(),
            sort: Default::default(),
        }
    }
}

#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum StreamContent {
    Error(StreamError),
    Data(String),
    Done,
}
