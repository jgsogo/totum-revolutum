use diesel::associations::HasTable;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::query_dsl::limit_dsl::LimitDsl;

/// Helper trait that adds a method to return objects from models using their primary key. This
/// should be blanked-implemented for the vast majority of diesel tables.
pub trait GetByPk<PK> {
    type QueryOutput;

    /// Returns a query to fetch the single row for the given primary key (`pk`)
    fn get_by_pk(pk: PK) -> Self::QueryOutput;
}

impl<PK, T> GetByPk<PK> for T
where
    // for<'a> &'a T: Identifiable, // Not really needed
    T: HasTable,
    T::Table: FindDsl<PK>,
    <<T as HasTable>::Table as FindDsl<PK>>::Output: LimitDsl,
{
    type QueryOutput = <<<T as HasTable>::Table as FindDsl<PK>>::Output as LimitDsl>::Output;

    fn get_by_pk(pk: PK) -> Self::QueryOutput {
        let table = T::table();
        let stm = table.find(pk);
        LimitDsl::limit(stm, 1)
    }
}
