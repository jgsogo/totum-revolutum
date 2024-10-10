//! These tests guarantee that the source code we write (and exercise in production using
//! the production database) works with the migrations and the auto-generated schema.
//!
//! Note that production code runs against PostgreSQL database, while testing uses SQlite3
//! and they have different database types.

mod test_account;
mod test_movement;
mod test_snapshot;
