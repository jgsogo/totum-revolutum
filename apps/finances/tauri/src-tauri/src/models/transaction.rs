use crate::models::movement::NewMovement;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewTransaction {
    pub name: String,
    pub description: Option<String>,
    pub transaction_group_pk: Option<i64>,
    pub movements_from: Vec<NewMovement>,
    pub movements_to: Vec<NewMovement>,
}
