use serde::{Deserialize, Serialize};

use crate::models::{AccountType, Holder};

type AccountAndRelatedData = (
    finances_db::models::Account,
    finances_db::models::AccountHolder,
    finances_db::models::AccountType,
);

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub pk: i32,
    pub name: String,
    pub holder: Holder,
    pub r#type: AccountType,
    pub ccy: String,
    pub identifier: Option<String>,
}

impl From<AccountAndRelatedData> for Account {
    fn from(value: AccountAndRelatedData) -> Self {
        let (account, holder, account_type) = value;
        Self {
            pk: account.id,
            name: account.name,
            holder: holder.into(),
            r#type: account_type.into(),
            ccy: account.ccy,
            identifier: account.identifier,
        }
    }
}
