//! Provide traits that generate queries. These queries can be modified, stored and even converted
//! to actual SQL statements, but they won't be actually executed. They correspond to the
//! [Query Object pattern (Martin Fowler)](https://martinfowler.com/eaaCatalog/queryObject.html)

mod filter_by_pk;
mod get_by_pk;

pub use filter_by_pk::FilterByPk;
pub use get_by_pk::GetByPk;
