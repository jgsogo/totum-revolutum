mod app_state;
mod holder;
mod main_context;
mod protos;

pub trait AppModel {
    fn encode_to_vec(&self) -> Vec<u8>;
}
pub use app_state::AppState;
pub use holder::Holder;
pub use main_context::MainContext;
