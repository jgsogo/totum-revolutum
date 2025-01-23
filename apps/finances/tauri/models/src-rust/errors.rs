use std::num::TryFromIntError;

use bigdecimal::ParseBigDecimalError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot convert from one type to the other: {0}")]
    ConversionError(ConversionError),

    #[error("Required field '{0}' is missing")]
    MissingRequiredField(String),

    #[error(transparent)]
    ProtoDecodeError(#[from] prost::DecodeError),
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error(transparent)]
    ParseBigDecimalError(#[from] ParseBigDecimalError),

    #[error("Invalid input date: year {year}, month {month}, day {day}")]
    FromDateComponents { year: i32, month: i32, day: i32 },

    #[error("Invalid currency code '{0}'")]
    InvalidCurrencyCode(String),

    #[error("Invalid movement direction '{0}'")]
    InvalidMovementDirection(i32),

    #[error("Cannot convert BigInt({0}) to i64")]
    I64Overflow(num_bigint::BigInt),

    #[error("Cannot convert BigInt({0}) to i32")]
    I32Overflow(num_bigint::BigInt),

    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),

    #[error("Failed to create BigInt from radix 10 buffer")]
    BigIntFromRadix10Error,
}
