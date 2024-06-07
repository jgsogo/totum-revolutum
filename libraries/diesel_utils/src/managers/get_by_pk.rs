use diesel::connection::LoadConnection;
use diesel::query_dsl::LoadQuery;
use diesel::RunQueryDsl;

pub trait GetByPkManager<PK, Conn>: Sized {
    type Error: From<diesel::result::Error>;

    fn get_by_pk(pk: PK, conn: &mut Conn) -> Result<Self, Self::Error>;
}

impl<PK, Conn, T: crate::queryset::GetByPk<PK>> GetByPkManager<PK, Conn> for T
where
    <T as crate::queryset::GetByPk<PK>>::QueryOutput: RunQueryDsl<Conn> + for<'a> LoadQuery<'a, Conn, T>,
    Conn: LoadConnection,
{
    type Error = crate::error::Error;

    fn get_by_pk(pk: PK, conn: &mut Conn) -> Result<Self, Self::Error> {
        let qs = <T as crate::queryset::GetByPk<PK>>::get_by_pk(pk);
        Ok(qs.get_result(conn)?)
    }
}
