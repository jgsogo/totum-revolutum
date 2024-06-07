#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

diesel::table! {
    directories (id) {
        id -> Integer,
        name -> Text,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Text,
        directory_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    directories,
    files,
);
