//! Contains type definitions used in the database and across the library that depend on the
//! active features

mod backend;
mod numeric;

pub use backend::BackendType;
pub use numeric::{compare_eq, Double, NumericType};
