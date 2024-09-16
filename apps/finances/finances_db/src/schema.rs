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
        // open -> Date,
        // close -> Nullable<Date>,
        holder_id -> Integer,
        type_id -> Integer,
    }
}


diesel::joinable!(data_account -> data_accounttype (type_id));
diesel::joinable!(data_account -> data_accountholder (holder_id));

diesel::allow_tables_to_appear_in_same_query!(
    data_account,
    data_accounttype,
    data_accountholder,
);
