use diesel::associations::HasTable;
use diesel::dsl::EqAny;
use diesel::internal::derives::multiconnection::array_comparison::AsInExpression;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::sql_types::SqlType;
use diesel::{ExpressionMethods, Table};

pub trait FilterByPk<'a, PK, Values> {
    type QueryOutput;

    fn filter_by_pk(pk: Values) -> Self::QueryOutput;
}

impl<'a, PK, T, Values> FilterByPk<'a, PK, Values> for T
where
    // &'a T: Identifiable, // Not really needed
    T: HasTable,
    T::Table: Table<PrimaryKey = PK>,
    PK: ExpressionMethods + 'a,
    <PK as diesel::Expression>::SqlType: SqlType,
    T::Table: FilterDsl<EqAny<PK, Values>>,
    Values: AsInExpression<<PK as diesel::Expression>::SqlType>,
{
    type QueryOutput = <<T as HasTable>::Table as FilterDsl<EqAny<PK, Values>>>::Output;

    fn filter_by_pk(pks: Values) -> Self::QueryOutput {
        let table = T::table();
        let primary_key = table.primary_key();
        let predicate = primary_key.eq_any(pks);
        FilterDsl::filter(table, predicate)
    }
}
