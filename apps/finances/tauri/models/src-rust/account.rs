use super::{google_type, AppModel, Custodian};

pub struct Account(crate::protos::Account);

impl Account {
    pub fn new(
        account: finances_accounts::models::Account,
        account_holder_role: finances_accounts::models::AccountHolderRole,
        custodian: Custodian,
        account_type: crate::protos::AccountType,
    ) -> Self {
        let open_date: google_type::Date = account.open.into();

        Self(crate::protos::Account {
            pk: account.id,
            name: account.name,
            custodian: Some(custodian.inner_type()),
            r#type: Some(account_type),
            currency_code: account.ccy,
            identifier: account.identifier,
            description: account.description,
            open: Some(open_date.into()),
            holder_owns_money: account_holder_role.owns_money,
            is_numerable: account.is_numerable,
        })
    }
    pub fn is_numerable(&self) -> bool {
        self.0.is_numerable
    }
    pub fn ccy(&self) -> &str {
        &self.0.currency_code
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
