#[cfg(feature = "sqlite")]
pub use diesel::sqlite::Sqlite as BackendType;

#[cfg(feature = "postgres")]
pub use diesel::pg::Pg as BackendType;
