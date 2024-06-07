//! Managers abstract the user from the underlying database. They provide useful functions that
//! return domain objects directly. Behind the scenes they can use querysets from
//! [`super::queryset`] to actually create the queries and execute them.
//!
//! We can think about them in terms of the **Repository pattern** as well.

pub use all::All;

mod all;
