mod movement_dividend;
pub(crate) mod movement_numerable;
pub(crate) mod snapshot_numerable;

pub use movement_dividend::{MovementDividend, NewMovementDividend};
pub use movement_numerable::{MovementNumerable, NewMovementNumerable};
pub use snapshot_numerable::{NewSnapshotNumerable, SnapshotNumerable};

/// A type that can hold all movement types
pub enum Movement {
    NonNumerable(finances_accounts::models::Movement),
    Numerable(MovementNumerable),
    Dividend(MovementDividend),
}

impl Movement {
    pub fn id(&self) -> i64 {
        match self {
            Movement::NonNumerable(movement) => movement.id,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.id,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.id,
        }
    }

    pub fn amount(&self) -> &finances_accounts::types::NumericType {
        match self {
            Movement::NonNumerable(movement) => &movement.amount,
            Movement::Numerable(movement_numerable) => &movement_numerable.movement.amount,
            Movement::Dividend(movement_dividend) => &movement_dividend.movement.amount,
        }
    }

    pub fn direction(&self) -> finances_accounts::fields::MovementDirection {
        match self {
            Movement::NonNumerable(movement) => movement.direction,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.direction,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.direction,
        }
    }
    pub fn date_value(&self) -> &chrono::NaiveDate {
        match self {
            Movement::NonNumerable(movement) => &movement.date_value,
            Movement::Numerable(movement_numerable) => &movement_numerable.movement.date_value,
            Movement::Dividend(movement_dividend) => &movement_dividend.movement.date_value,
        }
    }

    pub fn account_id(&self) -> i64 {
        match self {
            Movement::NonNumerable(movement) => movement.account_id,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.account_id,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.account_id,
        }
    }

    pub fn fx_id(&self) -> Option<i64> {
        match self {
            Movement::NonNumerable(movement) => movement.fx_id,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.fx_id,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.fx_id,
        }
    }

    pub fn type_id(&self) -> i64 {
        match self {
            Movement::NonNumerable(movement) => movement.type_id,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.type_id,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.type_id,
        }
    }

    pub fn transaction_id(&self) -> i64 {
        match self {
            Movement::NonNumerable(movement) => movement.transaction_id,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.transaction_id,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.transaction_id,
        }
    }
}
