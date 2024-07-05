use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct JSONField(serde_json::Value);

impl JSONField {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }

    pub fn as_json_value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl AsRef<serde_json::Value> for JSONField {
    fn as_ref(&self) -> &serde_json::Value {
        self.as_json_value()
    }
}

impl<DB: Backend> FromSql<Text, DB> for JSONField
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let t = String::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&t)?))
    }
}

impl ToSql<Text, Sqlite> for JSONField
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        out.set_value(s);
        Ok(IsNull::No)
    }
}
