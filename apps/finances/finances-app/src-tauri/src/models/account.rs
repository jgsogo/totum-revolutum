use crate::models::{AccountType, Holder};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Account {
    // pub pk: i32,
    pub name: String,
    pub holder: Holder,
    pub r#type: AccountType,
    pub ccy: String,
    pub identifier: Option<String>,
}

impl
    From<(
        finances_db::models::Account,
        finances_db::models::AccountHolder,
        finances_db::models::AccountType,
    )> for Account
{
    fn from(
        value: (
            finances_db::models::Account,
            finances_db::models::AccountHolder,
            finances_db::models::AccountType,
        ),
    ) -> Self {
        let (account, holder, account_type) = value;
        Self {
            // pk: account.id,
            name: account.name,
            holder: holder.into(),
            r#type: account_type.into(),
            ccy: account.ccy,
            identifier: account.identifier,
        }
    }
}
