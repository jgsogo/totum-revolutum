//! Provides Rust wrappers over some protobuf provided by `googleapis`
//!
//! FIXME: Move this to //libraries/googleapis and reuse it.

mod currency_code;
// mod date;
// mod decimal;
// mod money;
mod money_ref;

pub use currency_code::CurrencyCode;
// pub use date::Date;
// pub use decimal::Decimal;
// pub use money::Money;
pub use money_ref::MoneyRef;
