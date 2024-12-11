use crate::managers::create_movement_numerable;
use crate::models::MovementNumerable;
use crate::sql::queries::all_movementnumerable_for_account_id;
use crate::test_utils::establish_connection;
use diesel::prelude::*;
use finances_accounts::models::{Movement, MovementType, Transaction};
use finances_accounts::sql::filters::{movement_filter_account_by_pk, movementtype_by_unique_name};

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let account_id = 4;
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

    // All movements (as MovementNumerable) for a given account
    {
        let all: Vec<MovementNumerable> = all_movementnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(account_id)
            .load(&mut conn)
            .expect("Error loading numerable movements");
        assert_eq!(all.len(), 3);
    }

    // Insert one more movement numerable
    let amount: bigdecimal::BigDecimal = 3f32.try_into().unwrap();
    let date_value = chrono::NaiveDate::parse_from_str("2024-09-10", "%Y-%m-%d").unwrap();
    let quantity: bigdecimal::BigDecimal = 5f32.try_into().unwrap();
    let unit_value: bigdecimal::BigDecimal = 500f32.try_into().unwrap();
    let movement_type = MovementType::all()
        .filter(movementtype_by_unique_name(
            crate::constants::movementtype::TAXES_INCOME_AND_VALUE_DIRECT,
        ))
        .select(finances_accounts::schema::finances_accounts_movementtype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();
    create_movement_numerable(
        &mut conn,
        &amount,
        0,
        &date_value,
        &account_id,
        None,
        &movement_type,
        &0,
        &quantity,
        &unit_value,
    )
    .unwrap();

    let all: Vec<MovementNumerable> = all_movementnumerable_for_account_id()
        .bind::<diesel::sql_types::Int8, _>(account_id)
        .load(&mut conn)
        .expect("Error loading numerable movements");
    assert_eq!(all.len(), 4);

    // Fail to insert another movement
    {
        let r: Result<(), diesel::result::Error> = conn.transaction(|conn| {
            // Here we create a movement inside this transaction
            let r = create_movement_numerable(
                conn,
                &amount,
                0,
                &date_value,
                &account_id,
                None,
                &movement_type,
                &0,
                &quantity,
                &unit_value,
            );
            assert!(r.is_ok());
            // Here we raise an error inside the transaction
            Err(diesel::result::Error::NotFound)
        });
        assert!(r.is_err());
        assert!(matches!(r.unwrap_err(), diesel::result::Error::NotFound));

        // We should have the same 4 movements we had before
        let all: Vec<MovementNumerable> = all_movementnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(account_id)
            .load(&mut conn)
            .expect("Error loading numerable movements");
        assert_eq!(all.len(), 4);
    }
}
