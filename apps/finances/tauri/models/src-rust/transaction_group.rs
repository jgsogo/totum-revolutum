use super::AppModel;

pub struct TransactionGroup(crate::protos::TransactionGroup);

impl TransactionGroup {
    pub fn new(pk: i64, name: String, description: Option<String>) -> Self {
        Self(crate::protos::TransactionGroup { pk, name, description })
    }
}

impl AppModel<crate::protos::TransactionGroup> for TransactionGroup {
    fn inner_type(self) -> crate::protos::TransactionGroup {
        self.0
    }
}
