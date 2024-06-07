use diesel::query_dsl::LoadQuery;
use diesel::RunQueryDsl;

/// Declares a manager that executes [`crate::querysets::FilterByPkQuerySet`]
pub trait FilterByPkManager<PK, Values, Conn>: Sized {
    type Error: From<diesel::result::Error>;

    fn filter_by_pk(pks: Values, conn: &mut Conn) -> Result<Vec<Self>, Self::Error>;
}

impl<PK, Values, Conn, T: crate::querysets::FilterByPkQuerySet<PK, Values>> FilterByPkManager<PK, Values, Conn> for T
where
    <T as crate::querysets::FilterByPkQuerySet<PK, Values>>::QueryOutput:
        RunQueryDsl<Conn> + for<'a> LoadQuery<'a, Conn, T>,
    // These are needed only to get the size of the `pks: Values` argument
    // Values: IntoIterator,
    // Values: AsInExpression<<PK as diesel::Expression>::SqlType>,
    // PK: ExpressionMethods,
    // <PK as diesel::Expression>::SqlType: SqlType,
{
    type Error = crate::error::Error;

    fn filter_by_pk(pks: Values, conn: &mut Conn) -> Result<Vec<Self>, Self::Error> {
        // let pks = pks.into_iter().collect();
        // let _a =  pks.as_in_expression();
        let qs = <T as crate::querysets::FilterByPkQuerySet<PK, Values>>::filter_by_pk(pks);
        let r = qs.load::<T>(conn)?;
        // assert_eq!()
        // if r.len()
        Ok(r)
    }
}
