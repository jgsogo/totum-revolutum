use diesel::associations::HasTable;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::query_dsl::limit_dsl::LimitDsl;

// TODO: Write docs with example usage
pub trait GetByPk<PK> {
    type QueryOutput;

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
