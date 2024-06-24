#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

diesel::table! {
    directories (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        full_path -> Text,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Text,
        directory_id -> Integer,
        hash -> Text,
        size -> Integer,
        fileid -> Nullable<BigInt>,
        format_id -> Nullable<Integer>,
        processed -> Bool,
    }
}

diesel::table! {
    formats (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        format -> Text,
    }
}

diesel::table! {
    photos (id) {
        id -> Integer,
        fileid -> BigInt,
        path -> Text,
        processed -> Bool,
    }
}

diesel::joinable!(files -> directories (directory_id));
diesel::joinable!(files -> formats (format_id));

diesel::allow_tables_to_appear_in_same_query!(
    directories,
    files,
    formats,
    photos,
);
