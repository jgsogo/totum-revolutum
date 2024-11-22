use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ParentMovementType {
    Id(i32),
    MovementType(Box<MovementType>),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MovementType {
    // pub pk: i32,
    pub name: String,
    pub level: i16,
    pub parent: Option<ParentMovementType>,
}

impl From<finances_accounts::models::MovementType> for MovementType {
    fn from(value: finances_accounts::models::MovementType) -> Self {
        Self {
            name: value.name,
            level: value.level,
            parent: value.parent_id.map(ParentMovementType::Id),
        }
    }
}
