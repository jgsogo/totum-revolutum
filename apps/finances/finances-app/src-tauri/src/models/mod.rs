//! Defines all the models used by the application in the backend. These models are serialized
//! and sent to the frontend

mod account;
mod account_type;
mod holder;
mod snapshot;

pub use account::Account;
pub use account_type::AccountType;
pub use holder::Holder;
pub use snapshot::Snapshot;
