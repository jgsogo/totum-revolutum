pub struct TransactionGroup(pub(crate) crate::protos::finances_app_models::TransactionGroup);

impl TransactionGroup {
    pub fn new(pk: i64, name: String, description: Option<String>) -> Self {
        Self(crate::protos::finances_app_models::TransactionGroup { pk, name, description })
    }
}
