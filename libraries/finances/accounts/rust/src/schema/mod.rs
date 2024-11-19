#[cfg(feature = "postgres")]
mod schema_postgres;

#[cfg(feature = "postgres")]
pub use schema_postgres::*;
