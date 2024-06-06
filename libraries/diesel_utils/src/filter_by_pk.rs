use diesel::associations::HasTable;
use diesel::dsl::EqAny;
use diesel::query_builder::AsQuery;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::sql_types::SqlType;
use diesel::{ExpressionMethods, Table};

pub trait FilterByPk<'a, PK> {
    type QueryOutput;

    fn filter_by_pk(pk: &'a [PK]) -> Self::QueryOutput;
}

impl<'a, PK, T> FilterByPk<'a, PK> for T
where
    // for<'a> &'a T: Identifiable<Id=&'a PK>,
    T: HasTable,
    T::Table: Table<PrimaryKey = PK>, // + FilterDsl<EqAny<PK, dyn Iterator<Item=PK>>>,
    PK: ExpressionMethods + 'a,
    <PK as diesel::Expression>::SqlType: SqlType,
    <<T as HasTable>::Table as AsQuery>::Query: FilterDsl<EqAny<PK, &'a [PK]>>,
{
    type QueryOutput = ();

    fn filter_by_pk(pks: &'a [PK]) -> Self::QueryOutput {
        let table = T::table();
        let primary_key = table.primary_key();
        let predicate = primary_key.eq_any(pks);
        FilterDsl::filter(table, predicate);
        // table.filter(predicate);
        // todo!()
    }
}
