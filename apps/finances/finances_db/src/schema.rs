#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

// Use the following query to list columns and datatypes
// SELECT
//    column_name,
//    data_type
// FROM
//    information_schema.columns
// WHERE
//    table_name = 'data_account'
//    ;


diesel::table! {
    data_account (id) {
        id -> Integer,
        identifier -> Text,
        name -> Text,
        is_numerable -> Bool,
        ccy -> Text,
        // open -> Date,
        // close -> Date,
        holder_id -> Integer,
        type_id -> Integer,
    }
}
