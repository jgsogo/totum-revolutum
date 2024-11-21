//! Provides an override for the generated `Double` field in the `schema.rs` file
//!
//! I'm generating the `schema.rs` file using a Sqlite3 database, this database
//! doesn't have a `NUMERIC` data type, which is the one that appears in the
//! migrations SQL files. Because of this, in the generated `schema.rs` file
//! it appears as a `Double` type (see [SQLite affinity table](https://www.sqlite.org/datatype3.html)).
//!
//! In this module I redefine the `Double` type depending on the active feature:
//! * `sqlite` uses [`diesel::sql_types::Double`] and `f64`
//! * `poastgres` uses [`diesel::sql_types::Numeric`] and [`bigdecimal::BigDecimal`]

// #[cfg(feature = "sqlite")]
// pub use diesel::sql_types::Double;

// #[cfg(feature = "sqlite")]
// pub use f64 as NumericType;

#[cfg(feature = "postgres")]
pub use diesel::sql_types::Numeric as Double;

#[cfg(feature = "postgres")]
pub use bigdecimal::BigDecimal as NumericType;
