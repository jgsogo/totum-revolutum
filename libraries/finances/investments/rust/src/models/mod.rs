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
    pub fn date_value(&self) -> &chrono::NaiveDate {
        match self {
            Movement::NonNumerable(movement) => &movement.date_value,
            Movement::Numerable(movement_numerable) => &movement_numerable.movement.date_value,
            Movement::Dividend(movement_dividend) => &movement_dividend.movement.date_value,
        }
    }

    pub fn direction(&self) -> finances_accounts::fields::MovementDirection {
        match self {
            Movement::NonNumerable(movement) => movement.direction,
            Movement::Numerable(movement_numerable) => movement_numerable.movement.direction,
            Movement::Dividend(movement_dividend) => movement_dividend.movement.direction,
        }
    }
}
