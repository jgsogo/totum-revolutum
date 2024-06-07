use diesel::associations::HasTable;
use diesel::query_dsl::methods::SelectDsl;
use diesel::query_dsl::LoadQuery;
use diesel::{Expression, QueryDsl, RunQueryDsl};

// TODO: Write docs with example usage
pub trait All<Output, Selection: Expression, Conn> {
    type Error: From<diesel::result::Error>;

    fn all(selection: Selection, conn: &mut Conn) -> Result<impl Iterator<Item = Output>, Self::Error>;
}

impl<T, Output, Selection, Conn> All<Output, Selection, Conn> for T
where
    T: HasTable,
    Selection: Expression,
    T::Table: QueryDsl + SelectDsl<Selection>,
    // Conn: diesel::connection::LoadConnection,
    <<T as HasTable>::Table as SelectDsl<Selection>>::Output: RunQueryDsl<Conn>,
    for<'query> <<T as HasTable>::Table as SelectDsl<Selection>>::Output: LoadQuery<'query, Conn, Output>,
{
    type Error = crate::error::Error;

    fn all(selection: Selection, conn: &mut Conn) -> Result<impl Iterator<Item = Output>, Self::Error> {
        let table = T::table();
        /*
        fn select<Selection>(self, selection: Selection) -> Select<Self, Selection>
        where
            Selection: Expression,
            Self: methods::SelectDsl<Selection>,
        {
            methods::SelectDsl::select(self, selection)
        }

        impl<T, Selection> SelectDsl<Selection> for T
        where
            Selection: Expression,
            T: Table,
            T::Query: SelectDsl<Selection>,
        {
            type Output = <T::Query as SelectDsl<Selection>>::Output;

            fn select(self, selection: Selection) -> Self::Output {
                self.as_query().select(selection)
            }
        }


        */

        let r = SelectDsl::select(table, selection);
        /*
        fn load<'query, U>(self, conn: &mut Conn) -> QueryResult<Vec<U>>
        where
            Self: LoadQuery<'query, Conn, U>,
        {
            self.internal_load(conn)?.collect()
        }
        */
        let r = r.load(conn)?;
        Ok(r.into_iter())
    }
}
