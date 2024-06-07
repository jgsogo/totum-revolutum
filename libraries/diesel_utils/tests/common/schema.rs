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
    m2m_posts_tags (post_id, tag) {
        post_id -> Integer,
        tag -> Text,
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
    tags (tag) {
        tag -> Text,
        parent -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

diesel::joinable!(m2m_posts_tags -> posts (post_id));
diesel::joinable!(m2m_posts_tags -> tags (tag));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    m2m_posts_tags,
    posts,
    tags,
    users,
);
