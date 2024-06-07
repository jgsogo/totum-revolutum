use diesel::associations::HasTable;
use diesel::query_dsl::methods::SelectDsl;
use diesel::query_dsl::LoadQuery;
use diesel::{Expression, QueryDsl, RunQueryDsl};

/// Helper trait that adds a method to return all the objects of a given model. This should be
/// blanked-implemented for the vast majority of diesel tables.
// TODO: The `Output` type should be inferred from the `Selection`, it's known at compile time. If
// TODO: manage to define it here, the caller doesn't need to be explicit about the type returned
// TODO: in the iterator.
pub trait All<Output, Selection: Expression, Conn> {
    type Error: From<diesel::result::Error>;

    /// Returns an iterator with all the objects in the database
    fn all(selection: Selection, conn: &mut Conn) -> Result<impl Iterator<Item = Output>, Self::Error>;
}

impl<T, Output, Selection, Conn> All<Output, Selection, Conn> for T
where
    T: HasTable,
    Selection: Expression,
    T::Table: QueryDsl + SelectDsl<Selection>,
    <<T as HasTable>::Table as SelectDsl<Selection>>::Output: RunQueryDsl<Conn>,
    for<'query> <<T as HasTable>::Table as SelectDsl<Selection>>::Output: LoadQuery<'query, Conn, Output>,
{
    type Error = crate::error::Error;

    fn all(selection: Selection, conn: &mut Conn) -> Result<impl Iterator<Item = Output>, Self::Error> {
        let table = T::table();
        // TODO: Implement pagination here, that's the only reason why I'm passing the connection to
        // TODO: this function, so it can run several queries
        let r = SelectDsl::select(table, selection);
        let r = r.load(conn)?;
        Ok(r.into_iter())
    }
}
