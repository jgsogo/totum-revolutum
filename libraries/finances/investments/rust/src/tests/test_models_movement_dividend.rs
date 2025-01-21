use super::DB_POOL;
use crate::models::{MovementDividend, NewMovementDividend};
use crate::sql::queries::all_movementdividend_for_account_id;
use diesel::prelude::*;
use finances_accounts::fields::MovementDirection;
use finances_accounts::models::{Movement, MovementType, NewMovement, Transaction};
use finances_accounts::sql::filters::{movement_filter_account_by_pk, movementtype_by_unique_name};

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

    let account_id = 6;
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

        assert_eq!(all.len(), 3);
    }

    // All movements (as MovementDividend) for a given account
    {
        let all: Vec<MovementDividend> =
            all_movementdividend_for_account_id(account_id, &mut conn).expect("Error loading dividend movements");
        assert_eq!(all.len(), 0);
    }

    // Insert one movement dividend
    let transaction_id = Transaction::all()
        .select(finances_accounts::schema::finances_accounts_transaction::id)
        .first::<i64>(&mut conn)
        .unwrap();
    let amount: bigdecimal::BigDecimal = 3f32.try_into().unwrap();
    let date_value = chrono::NaiveDate::parse_from_str("2024-09-10", "%Y-%m-%d").unwrap();
    let ex_dividend_date = chrono::NaiveDate::parse_from_str("2024-09-01", "%Y-%m-%d").unwrap();
    let unit_value: bigdecimal::BigDecimal = 500f32.try_into().unwrap();
    let movement_type = MovementType::all()
        .filter(movementtype_by_unique_name(
            crate::constants::movementtype::INCOME_INVESTMENTS,
        ))
        .select(finances_accounts::schema::finances_accounts_movementtype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();

    let new_movement_dividend = NewMovementDividend {
        new_movement: &NewMovement {
            amount: &amount,
            direction: MovementDirection::In,
            date_value: &date_value,
            account_id: &account_id,
            fx_id: None,
            type_id: &movement_type,
            transaction_id: &transaction_id,
        },
        ex_dividend_date: &ex_dividend_date,
        unit_value: &unit_value,
    };
    new_movement_dividend.insert_into_db(&mut conn).unwrap();

    let all: Vec<MovementDividend> =
        all_movementdividend_for_account_id(account_id, &mut conn).expect("Error loading dividend movements");
    assert_eq!(all.len(), 1);
}
