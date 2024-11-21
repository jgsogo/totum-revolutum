// @generated automatically by Diesel CLI.

diesel::table! {
    finances_investments_movementdividend (movement_ptr_id) {
        movement_ptr_id -> Int8,
        ex_dividend_date -> Date,
        unit_value -> Numeric,
    }
}

diesel::table! {
    finances_investments_movementnumerable (movement_ptr_id) {
        movement_ptr_id -> Int8,
        quantity -> Numeric,
        unit_value -> Numeric,
    }
}

diesel::table! {
    finances_investments_snapshotnumerable (id) {
        id -> Int8,
        date_value -> Date,
        quantity -> Numeric,
        unit_value -> Numeric,
        account_id -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    finances_investments_movementdividend,
    finances_investments_movementnumerable,
    finances_investments_snapshotnumerable,
);
