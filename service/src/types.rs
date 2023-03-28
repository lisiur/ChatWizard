use diesel::backend::{Backend, RawValue};
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use diesel::{AsExpression, Expression, FromSqlRow};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{self, Uuid};

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Hash, Eq, PartialEq)]
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
