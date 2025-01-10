#[cfg(not(feature = "bazel"))]
pub mod google {
    pub mod r#type {
        include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
    }
}

#[cfg(feature = "bazel")]
pub mod google {
    pub mod r#type {
        pub use protos::google::r#type::{Date, Decimal, Money};
    }
}
