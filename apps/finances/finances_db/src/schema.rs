#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

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
    data_fx (id) {
        id -> Integer,
        foreign -> Text,
        local -> Text,
        rate -> Double,
        date_value -> Date,
    }
}

diesel::table! {
    data_movement (id) {
        id -> Integer,
        amount -> Double,
        quantity -> Nullable<Integer>,
        unit_value -> Nullable<Double>,
        date_value -> Date,
        account_id -> Integer,
        direction -> Integer,
        date -> Date,
        fx_id -> Nullable<Integer>,
        transfer_id -> Integer,
        type_id -> Integer,
    }
}

diesel::table! {
    data_movementtype (id) {
        id -> Integer,
        name -> Text,
        level -> SmallInt,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    data_snapshot (id) {
        id -> Integer,
        amount -> Double,
        quantity -> Nullable<Integer>,
        unit_value -> Nullable<Double>,
        date_value -> Date,
        account_id -> Integer,
    }
}

diesel::table! {
    data_transfer (id) {
        id -> Integer,
        description -> Text,
    }
}

diesel::joinable!(data_account -> data_accountholder (holder_id));
diesel::joinable!(data_account -> data_accounttype (type_id));
diesel::joinable!(data_movement -> data_account (account_id));
diesel::joinable!(data_movement -> data_fx (fx_id));
diesel::joinable!(data_movement -> data_movementtype (type_id));
diesel::joinable!(data_movement -> data_transfer (transfer_id));
diesel::joinable!(data_snapshot -> data_account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    data_account,
    data_accountholder,
    data_accounttype,
    data_fx,
    data_movement,
    data_movementtype,
    data_snapshot,
    data_transfer,
);
