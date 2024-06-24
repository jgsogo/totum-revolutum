#![cfg_attr(any(), rustfmt::skip)]
// @generated automatically by Diesel CLI.

diesel::table! {
    photos (id) {
        id -> Integer,
        fileid -> BigInt,
        path -> Text,
        processed -> Bool,
    }
}
