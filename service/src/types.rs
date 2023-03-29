use diesel::backend::{Backend, RawValue};
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::{Binary, Text};
use diesel::sqlite::Sqlite;
use diesel::{AsExpression, Expression, FromSqlRow};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{self, Uuid};

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

#[derive(AsExpression, FromSqlRow, Debug)]
#[diesel(sql_type = Text)]
pub struct TextWrapper<T: FromStr + AsRef<str> + fmt::Debug>(pub T);

impl<T: FromStr + AsRef<str> + fmt::Debug> FromSql<Text, Sqlite> for TextWrapper<T> {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        let string = String::from_utf8(bytes)?;
        let inner = T::from_str(&string).map_err(|_err| "error")?;
        Ok(TextWrapper(inner))
    }
}

impl<T: FromStr + AsRef<str> + fmt::Debug> ToSql<Text, Sqlite> for TextWrapper<T> {
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
pub struct JsonWrapper<T: fmt::Debug>(pub T);

impl<T: serde::de::DeserializeOwned + serde::Serialize + fmt::Debug> FromSql<Text, Sqlite>
    for JsonWrapper<T>
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        let inner = serde_json::from_slice::<T>(&bytes)?;
        Ok(JsonWrapper(inner))
    }
}

impl<T: serde::de::DeserializeOwned + serde::Serialize + fmt::Debug> ToSql<Text, Sqlite>
    for JsonWrapper<T>
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(serde_json::to_string(&self.0).unwrap());

        Ok(IsNull::No)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct PageQueryParams {
    pub page: i64,
    pub per_page: i64,
    pub query: Option<QueryParams>,
    pub sort: Option<Vec<Order>>,
}

impl Default for PageQueryParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
            query: None,
            sort: None,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub enum QueryParams {
    And(Box<QueryParams>, Box<QueryParams>),
    Or(Box<QueryParams>, Box<QueryParams>),
    Expr(QueryExpr),
}

#[derive(serde::Deserialize, Debug)]
pub enum QueryExpr {}

#[derive(serde::Deserialize, Debug)]
pub struct SortParams {
    column: String,
    order: Order,
}

#[derive(serde::Deserialize, Debug)]
pub enum Order {
    Asc,
    Desc,
}
