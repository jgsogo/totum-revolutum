#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

// Use the following query to list columns and datatypes:
// SELECT column_name, data_type FROM information_schema.columns WHERE table_name = 'data_account';
//
// Use the following to list all the tables
// SELECT * FROM pg_catalog.pg_tables;


diesel::table! {
    data_accountholder (id) {
        id -> Integer,
        name -> Text,
        owner -> Integer,
    }
}

diesel::table! {
    data_accounttype (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    data_account (id) {
        id -> Integer,
        identifier -> Nullable<Text>,
        name -> Text,
        is_numerable -> Bool,
        ccy -> Text,
        open -> Date,
        close -> Nullable<Date>,
        holder_id -> Integer,
        type_id -> Integer,
    }
}

diesel::table! {
    data_snapshot (id) {
        id -> Integer,
        amount -> Nullable<Numeric>,
        quantity -> Nullable<Integer>,
        unit_value -> Nullable<Numeric>,
        date_value -> Date,
        account_id -> Integer,
    }
}

diesel::table! {
    data_movementtype (id) {
        id -> Integer,
        name -> Text,
        level -> Integer,
        parent_id -> Integer,
    }
}

diesel::table! {
    data_movement (id) {
        id -> Integer,
        amount -> Numeric,
        quantity -> Integer,
        unit_value -> Numeric,
        direction -> Integer,
        date -> Date,
        date_value -> Date,
        account_id -> Integer,
        fx_id -> Integer,
        transfer_id -> Integer,
        type_id -> Integer, // movementtype
    }
}

diesel::table! {
    data_fx (id) {
        id -> Integer,
        foreign -> Text,
        local -> Text,
        rate -> Numeric,
        date_value -> Date,
    }
}

diesel::table! {
    data_transfer (id) {
        id -> Integer,
        description -> Text,
    }
}




diesel::joinable!(data_account -> data_accounttype (type_id));
diesel::joinable!(data_account -> data_accountholder (holder_id));
diesel::joinable!(data_snapshot -> data_account (account_id));
// diesel::joinable!(data_movementtype -> data_movementtype (parent_id));
diesel::joinable!(data_movement -> data_fx (fx_id));
diesel::joinable!(data_movement -> data_transfer (transfer_id));
diesel::joinable!(data_movement -> data_movementtype (type_id));



diesel::allow_tables_to_appear_in_same_query!(
    data_account,
    data_accounttype,
    data_accountholder,
);

diesel::allow_tables_to_appear_in_same_query!(
    data_snapshot,
    data_account,
);

diesel::allow_tables_to_appear_in_same_query!(
    data_movement,
    data_fx,
    data_transfer,
    data_movementtype,
);
