//! Defines all the models used by the application in the backend. These models are serialized
//! and sent to the frontend

mod movement;
mod new_amount;
mod snapshot;
mod transaction;

pub use movement::{NewMovement, NewMovementType};
pub use new_amount::NewAmount;
pub use snapshot::NewSnapshot;
pub use transaction::NewTransaction;
