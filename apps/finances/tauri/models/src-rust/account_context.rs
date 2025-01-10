use super::AppModel;

pub struct AccountContext(crate::protos::AccountContext);

impl AppModel<crate::protos::AccountContext> for AccountContext {
    fn inner_type(self) -> crate::protos::AccountContext {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::AccountContext {
        &self.0
    }
}
