use super::{Account, AccountType, Holder, MovementType, OutgoingModel, TransactionGroup};
use prost::Message;
pub struct MainContext(crate::protos::MainContext);

impl TryFrom<Vec<u8>> for MainContext {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::MainContext::decode(&*v)?))
    }
}

impl MainContext {
    pub fn new(
        holders: Vec<finances_accounts::models::AccountHolder>,
        account_types: Vec<AccountType>,
        movement_types: Vec<MovementType>,
        accounts: Vec<Account>,
        transaction_groups: Vec<TransactionGroup>,
    ) -> Self {
        Self(crate::protos::MainContext {
            holders: holders
                .into_iter()
                .map(|v| {
                    let holder: Holder = v.into();
                    holder.0
                })
                .collect(),
            account_types: account_types.into_iter().map(|v| v.0).collect(),
            movement_types: movement_types.into_iter().map(|v| v.0).collect(),
            accounts: accounts.into_iter().map(|v| v.0).collect(),
            transaction_groups: transaction_groups.into_iter().map(|v| v.0).collect(),
        })
    }

    pub fn find_account_type(&self, pk: i64) -> Option<AccountType> {
        let acc_type = self.0.account_types.iter().find(|acc_type| acc_type.pk == pk);
        acc_type.map(|v| AccountType(v.clone()))
    }

    pub fn find_movement_type(&self, pk: i64) -> Option<&crate::protos::MovementType> {
        self.0.movement_types.iter().find(|mov_type| mov_type.pk == pk)
    }

    pub fn find_account(&self, pk: i64) -> Option<&crate::protos::Account> {
        self.0.accounts.iter().find(|acc| acc.pk == pk)
    }

    pub fn find_movement_type_by_name(&self, name: &str) -> Option<MovementType> {
        self.0
            .movement_types
            .iter()
            .find(|mov_type| mov_type.name == name)
            .map(|v| MovementType(v.clone()))
    }
}

impl OutgoingModel for MainContext {
    fn encode_to_vec(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }
}
