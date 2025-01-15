#[cfg(not(feature = "bazel"))]
pub(crate) mod finances_app_models {
    include!(concat!(env!("OUT_DIR"), "/finances_app_models.rs"));
}

#[cfg(not(feature = "bazel"))]
pub(crate) mod google {
    pub(crate) mod r#type {
        include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
    }
}

#[cfg(feature = "bazel")]
pub(crate) mod finances_app_models {
    pub(crate) use protos::finances_app_models::{
        money_amount, Account, AccountCategory, AccountContext, AccountType, AppState, Custodian, DatabaseConnection,
        Fx, Holder, HolderContext, MainContext, MoneyAmount, Movement, MovementDirection, MovementType, NewSnapshot,
        Snapshot, TransactionGroup,
    };
}

#[cfg(feature = "bazel")]
pub(crate) mod google {
    pub(crate) mod r#type {
        pub(crate) use protos::date_proto::google::r#type::Date;
        pub(crate) use protos::decimal_proto::google::r#type::Decimal;
        pub(crate) use protos::money_proto::google::r#type::Money;
    }
}

// Reexport at root level
pub(crate) use finances_app_models::*;
