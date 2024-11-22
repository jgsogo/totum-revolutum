use crate::models::{MovementNumerable, MovementNumerableType};
use crate::test_utils::PopulateDatabase;
use diesel::prelude::*;
use finances_accounts::models::{Movement, MovementType, Transaction};
use finances_accounts::test_utils::fixtures::database_with_accounts;

#[test]
fn test_queries() {
    let mut database_with_accounts = database_with_accounts();
    database_with_accounts.populate_transactions().unwrap();

    let account_id = 0;
    database_with_accounts
        .populate_movements_numerable(account_id)
        .expect("Error populating database with MovementNumerable instances");
    database_with_accounts
        .populate_movements_numerable(1)
        .expect("Error populating database with MovementNumerable instances");

    {
        // TODO: Check that all movements for all acounts are equal to 4
        // let all = Movement::all()
        // assert_eq!(all.len(), 4);
    }

    // All movements (as Movement) for a given account
    {
        let all = Movement::all_with_related_data(account_id)
            .select((
                Movement::as_select(),
                Transaction::as_select(),
                MovementType::as_select(),
            ))
            .load::<(Movement, Transaction, MovementType)>(&mut database_with_accounts.conn)
            .expect("Error loading movements");

        assert_eq!(all.len(), 2);
    }

    // All movements (as MovementNumerable) for a given account
    {
        let all: Vec<MovementNumerableType> = MovementNumerable::all_with_related_data_raw(account_id)
            .bind::<diesel::sql_types::Int8, _>(account_id)
            .load(&mut database_with_accounts.conn)
            .expect("Error loading numerable movements");
        assert_eq!(all.len(), 2);
    }
}
