use super::account::AccountType;
use super::movement::MovementType;
use super::{Account, Holder, TransactionGroup};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct MainContext(crate::protos::finances_app_models::MainContext);

impl MainContext {
    pub fn new(
        holders: Vec<Holder>,
        account_types: Vec<AccountType>,
        movement_types: Vec<MovementType>,
        accounts: Vec<Account>,
        transaction_groups: Vec<TransactionGroup>,
    ) -> Self {
        Self(crate::protos::finances_app_models::MainContext {
            holders: holders.into_iter().map(|v| v.into()).collect(),
            account_types: account_types.into_iter().map(|v| v.into()).collect(),
            movement_types: movement_types.into_iter().map(|v| v.into()).collect(),
            accounts: accounts.into_iter().map(|v| v.into()).collect(),
            transaction_groups: transaction_groups.into_iter().map(|v| v.into()).collect(),
        })
    }

    pub fn holders(&self) -> Vec<&Holder> {
        self.0.holders.iter().map(Holder::new_ref).collect()
    }

    pub fn accounts(&self) -> Vec<&Account> {
        self.0.accounts.iter().map(Account::new_ref).collect()
    }

    pub fn find_account(&self, pk: i64) -> Option<&Account> {
        self.0.accounts.iter().find(|acc| acc.pk == pk).map(Account::new_ref)
    }

    pub fn find_account_type(&self, pk: i64) -> Option<&AccountType> {
        let acc_type = self.0.account_types.iter().find(|acc_type| acc_type.pk == pk);
        acc_type.map(AccountType::new_ref)
    }

    pub fn find_movement_type(&self, pk: i64) -> Option<&MovementType> {
        self.0
            .movement_types
            .iter()
            .find(|mov_type| mov_type.pk == pk)
            .map(MovementType::new_ref)
    }

    pub fn find_movement_type_by_name(&self, name: &str) -> Option<&MovementType> {
        self.0
            .movement_types
            .iter()
            .find(|mov_type| mov_type.name == name)
            .map(MovementType::new_ref)
    }
}
