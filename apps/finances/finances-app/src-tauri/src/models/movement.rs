use bigdecimal::ToPrimitive;
use serde::Serialize;

use crate::models::{Account, Fx, MovementType, Transfer};

type MovementAndRelatedData = (
    finances_db::models::Movement,
    finances_db::models::Fx,
    finances_db::models::Transfer,
    finances_db::models::MovementType,
);

#[derive(Serialize, Debug)]
pub enum MovementAccount {
    Id(i32),
    Account(Account),
}

#[derive(Serialize, Debug)]
pub struct Movement {
    // pub pk: i32,
    pub amount: f32,
    pub quantity: i32,
    pub unit_value: f32,
    pub direction: i32,
    pub date: String,
    pub date_value: String,
    pub account: MovementAccount,
    pub fx: Fx,
    pub transfer: Transfer,
    pub r#type: MovementType,
}

impl From<MovementAndRelatedData> for Movement {
    fn from(value: MovementAndRelatedData) -> Self {
        let (movement, fx, transfer, movement_type) = value;
        Self {
            amount: movement.amount.to_f32().unwrap(),
            quantity: movement.quantity,
            unit_value: movement.unit_value.to_f32().unwrap(),
            direction: movement.direction,
            date: movement.date.format("%Y-%m-%d").to_string(),
            date_value: movement.date_value.format("%Y-%m-%d").to_string(),
            account: MovementAccount::Id(movement.account_id),
            fx: fx.into(),
            transfer: transfer.into(),
            r#type: movement_type.into(),
        }
    }
}
