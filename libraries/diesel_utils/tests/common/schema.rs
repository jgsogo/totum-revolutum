#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Integer,
        post_id -> Integer,
        text -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
