use super::TransactionGroup;
pub struct Transaction(pub(crate) crate::protos::finances_app_models::Transaction);

impl Transaction {
    pub fn new(
        transaction: finances_accounts::models::Transaction,
        group: Option<finances_accounts::models::TransactionGroup>,
        movements: Vec<finances_investments::models::Movement>,
    ) -> Self {
        let (_movements_from, _movements_to): (Vec<_>, Vec<_>) = movements
            .into_iter()
            .partition(|mov| mov.direction() == finances_accounts::fields::MovementDirection::Out);

        Self(crate::protos::finances_app_models::Transaction {
            pk: transaction.id,
            name: transaction.name,
            description: transaction.description,
            group: group.map(|v| {
                let t: TransactionGroup = v.into();
                t.0
            }),
            movements_from: Vec::new(),
            movements_to: Vec::new(),
        })
    }
}
