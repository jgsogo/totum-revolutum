//! Provide all the protos used in this library

#[cfg(not(feature = "bazel"))]
pub mod finances_app_models {
    include!(concat!(env!("OUT_DIR"), "/finances_app_models.rs"));
}

#[cfg(not(feature = "bazel"))]
pub mod google {
    pub mod r#type {
        include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
    }
}

#[cfg(feature = "bazel")]
pub mod finances_app_models {
    pub use protos::finances_app_models::{
        money_amount, movement_amount, Account, AccountCategory, AccountContext, AccountType, AppState, Custodian,
        DatabaseConnection, FxQuote, Holder, HolderContext, LastTransactionsRequest, LastTransactionsResponse,
        MainContext, MoneyAmount, Movement, MovementAmount, MovementDirection, MovementType, Snapshot,
        SnapshotsRequest, SnapshotsResponse, Transaction, TransactionGroup,
    };
}

#[cfg(feature = "bazel")]
pub mod google {
    pub mod r#type {
        pub use protos::date_proto::google::r#type::Date;
        pub use protos::decimal_proto::google::r#type::Decimal;
        pub use protos::money_proto::google::r#type::Money;
    }
}
