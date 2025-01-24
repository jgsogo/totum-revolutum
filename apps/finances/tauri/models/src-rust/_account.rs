use super::{google_type, AccountType, Custodian};

pub struct Account(pub(crate) crate::protos::finances_app_models::Account);

impl From<Account> for crate::protos::finances_app_models::Account {
    fn from(val: Account) -> Self {
        val.0
    }
}

impl Account {
    pub fn new(
        account: finances_accounts::models::Account,
        account_holder_role: finances_accounts::models::AccountHolderRole,
        custodian: Custodian,
        account_type: AccountType,
    ) -> Self {
        let open_date: google_type::Date = account.open.into();

        Self(crate::protos::finances_app_models::Account {
            pk: account.id,
            name: account.name,
            custodian: Some(custodian.0),
            r#type: Some(account_type.0),
            currency_code: account.ccy,
            identifier: account.identifier,
            description: account.description,
            open: Some(open_date.0),
            holder_owns_money: account_holder_role.owns_money,
            is_numerable: account.is_numerable,
        })
    }
    pub fn pk(&self) -> i64 {
        self.0.pk
    }

    pub fn is_numerable(&self) -> bool {
        self.0.is_numerable
    }
    pub fn ccy(&self) -> &str {
        &self.0.currency_code
    }
}
