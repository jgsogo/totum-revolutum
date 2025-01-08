#[cfg(not(feature = "bazel"))]
include!(concat!(env!("OUT_DIR"), "/finances_app_models.rs"));

#[cfg(feature = "bazel")]
pub(crate) use protos::finances_app_models::{
    Account, AccountCategory, AccountType, AppState, Ccy, Custodian, DatabaseConnection, Holder, HolderContext,
    MainContext,
};
