// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod protos {
    #[cfg(not(feature = "bazel"))]
    include!(concat!(env!("OUT_DIR"), "/finances_app.rs"));

    #[cfg(feature = "bazel")]
    pub use account_proto::finances_app::Account;
}

#[cfg(test)]
mod tests {
    use super::*;

    use prost::Message;

    #[test]
    fn test_account() {
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
