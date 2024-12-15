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

/// Compares that two [`bigdecimal::BigDecimal`] scaling them first to 4 decimal digits (rounding [`bigdecimal::rounding::RoundingMode::HalfUp`])
pub fn compare_eq(lhs: &bigdecimal::BigDecimal, rhs: &bigdecimal::BigDecimal) -> bool {
    let lhs = lhs.with_scale_round(4, bigdecimal::rounding::RoundingMode::HalfUp);
    let rhs = rhs.with_scale_round(4, bigdecimal::rounding::RoundingMode::HalfUp);
    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_eq() {
        let lhs: bigdecimal::BigDecimal = 119.99999f32.try_into().unwrap();
        let rhs: bigdecimal::BigDecimal = 120f32.try_into().unwrap();
        assert!(compare_eq(&lhs, &rhs));

        let lhs: bigdecimal::BigDecimal = 119.9999f32.try_into().unwrap();
        let rhs: bigdecimal::BigDecimal = 120f32.try_into().unwrap();
        assert!(!compare_eq(&lhs, &rhs));
    }
}
