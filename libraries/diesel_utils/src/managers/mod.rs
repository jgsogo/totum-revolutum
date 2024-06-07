//! Managers abstract the user from the underlying database. They provide useful functions that
//! return domain objects directly. Behind the scenes they can use querysets from
//! [`super::querysets`] to actually create the queries and execute them.
//!
//! We can think about them in terms of the **Repository pattern** as well.

pub use all::AllManager;
pub use filter_by_pk::FilterByPkManager;
pub use get_by_pk::GetByPkManager;

mod all;
mod filter_by_pk;
mod get_by_pk;
