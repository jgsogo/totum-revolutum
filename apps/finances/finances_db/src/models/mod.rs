pub use account::Account;
pub use account_holder::AccountHolder;
pub use account_type::AccountType;
pub use fx::Fx;
pub use movement::Movement;
pub use movement_type::MovementType;
pub use snapshot::Snapshot;
pub use transfer::Transfer;

mod account;
mod account_holder;
mod account_type;
mod fx;
mod movement;
mod movement_type;
mod snapshot;
mod transfer;

pub mod types {
    /// Overrides the `Double` datatype that is generated from the SQL type `NUMERIC(14,4)`
    /// when using Sqlite3 (see migrations and compare with the generated `schema.rs`).
    ///
    /// The issue here is that the production database is PostgreSQL which implements the
    /// `NUMERIC` type, so when running against PostgreSQL I need it to be converted to
    /// BigDecimal, while it should be handled as a Double/f64 while using SQlite3
    ///
    /// So we are providing two different implementations of this `Double` type. When using
    /// sqlite it will just be an alias to [`diesel::sql_types::Double`], if using Postgresql
    /// it will be an alias to [`diesel::sql_types::Numeric`].
    ///
    /// The user needs to select the right feature to activate the right alias.

    #[cfg(feature = "sqlite")]
    pub use diesel::sql_types::Double;

    #[cfg(feature = "sqlite")]
    pub use f64 as NumericType;

    #[cfg(feature = "postgres")]
    pub use diesel::sql_types::Numeric as Double;

    #[cfg(feature = "postgres")]
    pub use bigdecimal::BigDecimal as NumericType;
}
