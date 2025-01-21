use super::DB_POOL;
use crate::models::{Movement, MovementDividend, MovementNumerable};
use crate::sql::queries::{
    all_movementdividend_for_account_id, all_movementnumerable_for_account_id, all_movements_for_account_id,
};
use diesel::prelude::*;

use finances_accounts::models::{MovementType, Transaction};
use finances_accounts::sql::filters::movement_filter_account_by_pk;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

    let account_id = 9;

    // All movements (as Movement) for a given account: includes regular movements, numerable and non-numerable ones
    {
        let all = finances_accounts::models::Movement::all()
            .filter(movement_filter_account_by_pk(account_id))
            .inner_join(Transaction::all())
            .inner_join(MovementType::all())
            .select((
                finances_accounts::models::Movement::as_select(),
                Transaction::as_select(),
                MovementType::as_select(),
            ))
            .load::<(finances_accounts::models::Movement, Transaction, MovementType)>(&mut conn)
            .expect("Error loading movements");

        assert_eq!(all.len(), 8);
    }

    // All movements (as MovementDividend) for a given account
    {
        let all: Vec<MovementDividend> =
            all_movementdividend_for_account_id(account_id, &mut conn).expect("Error loading dividend movements");
        assert_eq!(all.len(), 3);
    }

    // All movements (as MovementNumerable) for a given account
    {
        let all: Vec<MovementNumerable> =
            all_movementnumerable_for_account_id(account_id, &mut conn).expect("Error loading numerable movements");
        assert_eq!(all.len(), 3);
    }

    // All movements combined for a given account
    {
        let all: Vec<Movement> =
            all_movements_for_account_id(account_id, &mut conn).expect("Error loading all movements");
        assert_eq!(all.len(), 8);

        // Check they are ordered
        assert!(all.windows(2).all(|movs| {
            let lhs = &movs[0];
            let rhs = &movs[1];
            lhs.date_value() >= rhs.date_value()
        }))
    }
}
