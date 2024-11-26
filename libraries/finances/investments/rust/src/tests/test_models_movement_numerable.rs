use crate::models::MovementNumerable;
use crate::sql::queries::all_movementnumerable_for_account_id;
use crate::test_utils::establish_connection;
use diesel::prelude::*;
use finances_accounts::models::{Movement, MovementType, Transaction};
use finances_accounts::sql::filters::movement_filter_account_by_pk;

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let account_id = 0;
    {
        // TODO: Check that all movements for all acounts are equal to 4
        // let all = Movement::all()
        // assert_eq!(all.len(), 4);
    }

    // All movements (as Movement) for a given account
    {
        // TODO: Move these tests to finances_accounts
        let all = Movement::all()
            .filter(movement_filter_account_by_pk(account_id))
            .inner_join(Transaction::all())
            .inner_join(MovementType::all())
            .select((
                Movement::as_select(),
                Transaction::as_select(),
                MovementType::as_select(),
            ))
            .load::<(Movement, Transaction, MovementType)>(&mut conn)
            .expect("Error loading movements");

        assert_eq!(all.len(), 2);
    }

    // All movements (as MovementNumerable) for a given account
    {
        let all: Vec<MovementNumerable> = all_movementnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(account_id)
            .load(&mut conn)
            .expect("Error loading numerable movements");
        assert_eq!(all.len(), 2);
    }
}
