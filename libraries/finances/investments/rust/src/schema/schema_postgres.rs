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
    finances_investments_snapshotnumerable (snapshot_ptr_id) {
        snapshot_ptr_id -> Int8,
        quantity -> Numeric,
        unit_value -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    finances_investments_movementdividend,
    finances_investments_movementnumerable,
    finances_investments_snapshotnumerable,
);
