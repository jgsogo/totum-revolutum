use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct TransactionGroup(crate::protos::finances_app_models::TransactionGroup);

impl TransactionGroup {
    pub fn new(pk: i64, name: String, description: Option<String>) -> Self {
        Self(crate::protos::finances_app_models::TransactionGroup { pk, name, description })
    }
    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }
}
