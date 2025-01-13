mod account;
mod account_context;
mod account_type;
mod app_state;
mod custodian;
mod fx;
pub mod google_type;
mod holder;
mod holder_context;
mod main_context;
mod money_amount;
mod movement;
mod movement_type;
mod protos;
mod snapshot;
mod transaction_group;

pub use account::Account;
pub use account_context::AccountContext;
pub use account_type::{AccountCategory, AccountType};
pub use app_state::AppState;
pub use custodian::Custodian;
pub use fx::Fx;
pub use holder::Holder;
pub use holder_context::HolderContext;
pub use main_context::MainContext;
pub use money_amount::MoneyAmount;
pub use movement::{Movement, MovementDirection};
pub use movement_type::MovementType;
pub use snapshot::Snapshot;
pub use transaction_group::TransactionGroup;

pub trait AppModel<T: prost::Message> {
    fn inner_type(self) -> T;

    fn inner_type_ref(&self) -> &T;
}
