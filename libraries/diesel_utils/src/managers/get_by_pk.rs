use crate::error::{Error, ObjectDoesNotExist};
use diesel::connection::LoadConnection;
use diesel::query_dsl::LoadQuery;
use diesel::RunQueryDsl;
use std::any::type_name;

/// Declares a manager that executes [`crate::querysets::GetByPkQuerySet`]
pub trait GetByPkManager<PK, Conn>: Sized {
    type Error: From<diesel::result::Error>;

    /// Returns the object matching the primary key `pk`
    fn get_by_pk(pk: PK, conn: &mut Conn) -> Result<Self, Self::Error>;
}

impl<PK, Conn, T: crate::querysets::GetByPkQuerySet<PK>> GetByPkManager<PK, Conn> for T
where
    <T as crate::querysets::GetByPkQuerySet<PK>>::QueryOutput: RunQueryDsl<Conn> + for<'a> LoadQuery<'a, Conn, T>,
    Conn: LoadConnection,
{
    type Error = crate::error::Error;

    fn get_by_pk(pk: PK, conn: &mut Conn) -> Result<Self, Self::Error> {
        let qs = <T as crate::querysets::GetByPkQuerySet<PK>>::get_by_pk(pk);
        qs.get_result(conn).map_err(|e| match e {
            diesel::result::Error::NotFound => Error::ObjectDoesNotExist(ObjectDoesNotExist {
                model: type_name::<T>().to_string(),
            }),
            e => e.into(),
        })
    }
}
