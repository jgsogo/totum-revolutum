use diesel::associations::HasTable;
use diesel::dsl::EqAny;
use diesel::internal::derives::multiconnection::array_comparison::AsInExpression;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::sql_types::SqlType;
use diesel::{ExpressionMethods, Table};

// TODO: Write docs with example usage
pub trait FilterByPk<PK, Values> {
    type QueryOutput;

    fn filter_by_pk(pk: Values) -> Self::QueryOutput;
}

impl<PK, T, Values> FilterByPk<PK, Values> for T
where
    // &'a T: Identifiable, // Not really needed
    T: HasTable,
    T::Table: Table<PrimaryKey = PK> + FilterDsl<EqAny<PK, Values>>,
    PK: ExpressionMethods,
    <PK as diesel::Expression>::SqlType: SqlType,
    Values: AsInExpression<<PK as diesel::Expression>::SqlType>,
{
    type QueryOutput = <<T as HasTable>::Table as FilterDsl<EqAny<PK, Values>>>::Output;

    fn filter_by_pk(pks: Values) -> Self::QueryOutput {
        let table = T::table();
        let primary_key = table.primary_key();
        let predicate = primary_key.eq_any(pks);
        table.filter(predicate)
    }
}
