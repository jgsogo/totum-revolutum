//! A module with constants that are computed at compile time,
//! taken from <https://stackoverflow.com/questions/65120456/how-to-initialize-immutable-globals-with-non-const-initializer-in-rust>

extern crate proc_macro;
use proc_macro::TokenStream;
use std::f32::consts::FRAC_PI_3;

#[proc_macro]
pub fn sin60f32(_item: TokenStream) -> TokenStream {
    let p = FRAC_PI_3.sin();
    format!("{}f32", p).parse().unwrap()
}
