#[cfg(not(feature = "bazel"))]
pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/finances_app_models.rs"));
}

#[cfg(feature = "bazel")]
pub mod protos {
    pub use protos::finances_app_models::{Account, AppConfig, Ccy, DatabaseConnection, Holder, HolderList};
}

#[cfg(test)]
mod tests {
    use super::*;

    use prost::Message;

    #[test]
    fn test_account_roundtrip() {
        let account = protos::Account {
            name: "name".to_string(),
            description: None,
            open: None,
        };

        let encoded = account.encode_to_vec();

        let mut buf = encoded.as_slice();
        let roundtrip = protos::Account::decode(&mut buf).unwrap();
        assert_eq!(account.name, roundtrip.name);
    }
}
