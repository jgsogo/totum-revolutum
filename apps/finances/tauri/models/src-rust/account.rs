use super::{AppModel, Custodian};
use chrono::Datelike;

pub struct Account(crate::protos::Account);

impl Account {
    pub fn new(
        account: finances_accounts::models::Account,
        account_holder_role: finances_accounts::models::AccountHolderRole,
        custodian: Custodian,
        account_type_pk: i64,
    ) -> Self {
        let timestamp = crate::protos::Timestamp {
            seconds: (account.open.num_days_from_ce() * 24 * 3600) as i64,
            nanos: 0,
        };
        let ccy = crate::protos::Ccy::from_str_name(&account.ccy)
            .expect(&format!("{} is not a valid CCY", account.ccy))
            .into();

        Self(crate::protos::Account {
            pk: account.id,
            name: account.name,
            custodian: Some(custodian.inner_type()),
            account_type_pk,
            ccy,
            identifier: account.identifier,
            description: account.description,
            open: Some(timestamp),
            holder_owns_money: account_holder_role.owns_money,
            is_numerable: account.is_numerable,
        })
    }
}

impl AppModel<crate::protos::Account> for Account {
    fn inner_type(self) -> crate::protos::Account {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::Account {
        &self.0
    }
}
