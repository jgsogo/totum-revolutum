use serde::{Deserialize, Serialize};

use crate::models::{AccountType, Custodian};

type AccountAndRelatedData = (
    finances_accounts::models::Account,
    finances_accounts::models::AccountHolderRole,
    finances_accounts::models::Custodian,
    finances_accounts::models::AccountType,
);

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub pk: i64,
    pub name: String,
    pub custodian: Custodian,
    pub r#type: AccountType,
    pub ccy: String,
    pub identifier: Option<String>,
    pub holder_owns_money: bool,
    pub is_numerable: bool,
}

impl From<AccountAndRelatedData> for Account {
    fn from(value: AccountAndRelatedData) -> Self {
        let (account, account_holder_role, custodian, account_type) = value;
        Self {
            pk: account.id,
            name: account.name,
            custodian: custodian.into(),
            r#type: account_type.into(),
            ccy: account.ccy,
            identifier: account.identifier,
            holder_owns_money: account_holder_role.owns_money,
            is_numerable: account.is_numerable,
        }
    }
}
