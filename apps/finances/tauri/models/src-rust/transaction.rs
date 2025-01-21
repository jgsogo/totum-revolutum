use super::{Movement, MovementDirection, TransactionGroup};
pub struct Transaction(pub(crate) crate::protos::finances_app_models::Transaction);

impl Transaction {
    pub fn new(
        transaction: finances_accounts::models::Transaction,
        group: Option<finances_accounts::models::TransactionGroup>,
        movements: Vec<Movement>,
    ) -> Self {
        let (movements_from, movements_to): (Vec<_>, Vec<_>) = movements.into_iter().partition(|mov| {
            let direction: MovementDirection = mov
                .0
                .direction
                .try_into()
                .expect("Unexpected i32 for MovementDirection");
            direction == MovementDirection::Out
        });

        Self(crate::protos::finances_app_models::Transaction {
            pk: transaction.id,
            name: transaction.name,
            description: transaction.description,
            group: group.map(|v| {
                let t: TransactionGroup = v.into();
                t.0
            }),
            movements_from: movements_from.into_iter().map(|v| v.0).collect(),
            movements_to: movements_to.into_iter().map(|v| v.0).collect(),
        })
    }
}
