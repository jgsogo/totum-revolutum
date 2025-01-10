#[cfg(not(feature = "bazel"))]
pub(crate) mod finances_app_models {
    include!(concat!(env!("OUT_DIR"), "/finances_app_models.rs"));
}

#[cfg(not(feature = "bazel"))]
pub(crate) mod google {
    pub(crate) mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
    pub(crate) mod r#type {
        include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
    }
}

#[cfg(feature = "bazel")]
pub(crate) mod finances_app_models {
    pub(crate) use protos::finances_app_models::{
        Account, AccountCategory, AccountContext, AccountType, AppState, Ccy, Custodian, DatabaseConnection, Holder,
        HolderContext, MainContext,
    };
}

#[cfg(feature = "bazel")]
pub(crate) mod google {
    pub(crate) mod protobuf {
        pub(crate) use protos::timestamp_proto::google::protobuf::Timestamp;
    }
}

// Reexport at root level
pub(crate) use finances_app_models::*;
pub(crate) use google::protobuf::Timestamp;
