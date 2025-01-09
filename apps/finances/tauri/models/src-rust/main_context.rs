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

    pub fn find_account_type(&self, pk: i64) -> Option<&crate::protos::AccountType> {
        self.0.account_types.iter().find(|acc_type| acc_type.pk == pk)
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
