use super::{AccountType, AppModel, Holder};

pub struct MainContext(crate::protos::MainContext);

impl MainContext {
    pub fn new(holders: Vec<finances_accounts::models::AccountHolder>, account_types: Vec<AccountType>) -> Self {
        Self(crate::protos::MainContext {
            holders: holders
                .into_iter()
                .map(|v| {
                    let holder: Holder = v.into();
                    holder.inner_type()
                })
                .collect(),
            account_types: account_types.into_iter().map(|v| v.inner_type()).collect(),
        })
    }
}

impl AppModel<crate::protos::MainContext> for MainContext {
    fn inner_type(self) -> crate::protos::MainContext {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::MainContext {
        &self.0
    }
}
