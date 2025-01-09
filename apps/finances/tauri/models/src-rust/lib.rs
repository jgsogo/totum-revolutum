mod account;
mod account_type;
mod app_state;
mod custodian;
mod holder;
mod holder_context;
mod main_context;
mod protos;

pub use account::Account;
pub use account_type::{AccountCategory, AccountType};
pub use app_state::AppState;
pub use custodian::Custodian;
pub use holder::Holder;
pub use holder_context::HolderContext;
pub use main_context::MainContext;

pub trait AppModel<T: prost::Message> {
    fn inner_type(self) -> T;

    fn inner_type_ref(&self) -> &T;
}
