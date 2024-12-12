use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::serialize::{IsNull, Output};
use diesel::sql_types::Int4;
use diesel::AsExpression;
use diesel::FromSqlRow;

#[repr(i32)]
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Int4)]
pub enum MovementDirection {
    In = 0,
    Out = 1,
}

// impl<DB> ToSql<Int4, DB> for MovementDirection
// where
//     DB: Backend,
//     i32: ToSql<Int4, DB>,
// {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
//         match self {
//             MovementDirection::In => 0.to_sql(out),
//             MovementDirection::Out => 1.to_sql(out),
//         }
//     }
// }

impl ToSql<Int4, diesel::pg::Pg> for MovementDirection
where
    i32: ToSql<Int4, diesel::pg::Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        let v = *self as i32;
        <i32 as ToSql<Int4, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl ToSql<Int4, diesel::sqlite::Sqlite> for MovementDirection
where
    i32: ToSql<Int4, diesel::sqlite::Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(*self as i32);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Int4, DB> for MovementDirection
where
    DB: Backend,
    i32: FromSql<Int4, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(MovementDirection::In),
            1 => Ok(MovementDirection::Out),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}
