use diesel::associations::HasTable;
use diesel::dsl::EqAny;
use diesel::internal::derives::multiconnection::array_comparison::AsInExpression;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::sql_types::SqlType;
use diesel::{ExpressionMethods, Table};

/// Helper trait that adds a method to return all the objects for a given set of primary key
/// values. This should be blanked-implemented for the vast majority of diesel tables.
pub trait FilterByPk<PK, Values> {
    type QueryOutput;

    /// Returns a query to get all the rows for the given set of primary keys (`pks`)
    fn filter_by_pk(pks: Values) -> Self::QueryOutput;
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
