use std::any::type_name;

use diesel::query_dsl::LoadQuery;
use diesel::RunQueryDsl;
use log::warn;

use crate::error::Error;

/// Declares a manager that executes [`crate::querysets::FilterByPkQuerySet`]
pub trait FilterByPkManager<PK, Values, Conn>: Sized {
    type Error: From<diesel::result::Error>;

    /// Returns the objects identified by the given primary keys `pks`. This function will check
    /// that there are no duplicates in the input list (warning) and will fail if it finds fewer
    /// objects than input pks ([`Error::ObjectDoesNotExist`]).
    fn filter_by_pk(pks: Values, conn: &mut Conn) -> Result<Vec<Self>, Self::Error>;
}

impl<'a, PK, Values, Conn, T: crate::querysets::FilterByPkQuerySet<PK, Vec<<Values as IntoIterator>::Item>>>
    FilterByPkManager<PK, Values, Conn> for T
where
    <T as crate::querysets::FilterByPkQuerySet<PK, Vec<<Values as IntoIterator>::Item>>>::QueryOutput:
        RunQueryDsl<Conn> + LoadQuery<'a, Conn, T>,
    // Needed to count the input pks and validation
    Values: IntoIterator,
    <Values as IntoIterator>::Item: Ord + PartialEq,
{
    type Error = crate::error::Error;

    fn filter_by_pk(pks: Values, conn: &mut Conn) -> Result<Vec<Self>, Self::Error> {
        // Get the size of input `pks`
        let mut values: Vec<_> = pks.into_iter().collect();
        let mut expected_len = values.len();
        values.sort();
        values.dedup();
        if expected_len > values.len() {
            expected_len = values.len();
            warn!("Duplicated pks removed");
        }

        // execute the query
        let qs =
            <T as crate::querysets::FilterByPkQuerySet<PK, Vec<<Values as IntoIterator>::Item>>>::filter_by_pk(values);
        let r = qs.load::<T>(conn)?;

        // validate output
        assert!(r.len() <= expected_len, "Never more results. It's a PK!");
        if r.len() < expected_len {
            Err(Error::ObjectDoesNotExist(crate::error::ObjectDoesNotExist {
                model: type_name::<T>().to_string(),
            }))
        } else {
            Ok(r)
        }
    }
}
