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
    photo_files (file_id) {
        file_id -> Integer,
        fileid -> BigInt,
        format_id -> Integer,
        processed -> Bool,
    }
}

diesel::joinable!(files -> directories (directory_id));
diesel::joinable!(photo_files -> files (file_id));
diesel::joinable!(photo_files -> formats (format_id));

diesel::allow_tables_to_appear_in_same_query!(
    directories,
    files,
    formats,
    photo_files,
);
