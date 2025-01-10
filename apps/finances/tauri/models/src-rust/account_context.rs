use super::{Account, AppModel, Movement, Snapshot};

pub struct AccountContext(crate::protos::AccountContext);

impl AccountContext {
    pub fn new(account: Account, movements: Vec<Movement>, snapshots: Vec<Snapshot>) -> Self {
        Self(crate::protos::AccountContext {
            account: Some(account.inner_type()),
            movements: movements.into_iter().map(|v| v.inner_type()).collect(),
            snapshots: snapshots.into_iter().map(|v| v.inner_type()).collect(),
        })
    }
}

impl AppModel<crate::protos::AccountContext> for AccountContext {
    fn inner_type(self) -> crate::protos::AccountContext {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::AccountContext {
        &self.0
    }
}
