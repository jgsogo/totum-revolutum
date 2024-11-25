use anyhow::Result;
use diesel::prelude::*;
use finances_accounts::test_utils::TestDatabase;
use finances_accounts::types::NumericType;

pub trait PopulateDatabase {
    fn populate_movements_numerable(&mut self, account_pk: i64) -> Result<Vec<i64>>;
}

impl PopulateDatabase for TestDatabase {
    fn populate_movements_numerable(&mut self, account_pk: i64) -> Result<Vec<i64>> {
        let movement_ids = self.populate_movements(account_pk)?;

        // _Convert_ them into MovementNumerable by creating an entry in the corresponding table
        use crate::schema::finances_investments_movementnumerable::dsl::*;

        let movement_numerables = movement_ids
            .into_iter()
            .map(|id| {
                (
                    movement_ptr_id.eq(id),
                    quantity.eq::<NumericType>(1.into()),
                    unit_value.eq::<NumericType>(100.into()),
                )
            })
            .collect::<Vec<_>>();

        let mut conn = self.pool.get()?;
        let results: Vec<i64> = diesel::insert_into(finances_investments_movementnumerable)
            .values(&movement_numerables)
            .returning(movement_ptr_id)
            .get_results(&mut conn)?;

        Ok(results)
    }
}
