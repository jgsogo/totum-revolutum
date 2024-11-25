//! Contains type definitions used in the database and across the library that depend on the
//! active features

// #[cfg(feature = "sqlite")]
// pub use diesel::sqlite::SqliteConnection as ConnectionType;

#[cfg(feature = "postgres")]
pub use diesel::pg::PgConnection as ConnectionType;
