pub struct TransactionGroup(pub(crate) crate::protos::TransactionGroup);

impl TransactionGroup {
    pub fn new(pk: i64, name: String, description: Option<String>) -> Self {
        Self(crate::protos::TransactionGroup { pk, name, description })
    }
}
