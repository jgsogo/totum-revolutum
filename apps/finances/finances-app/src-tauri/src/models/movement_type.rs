use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ParentMovementType {
    Id(i32),
    MovementType(Box<MovementType>),
}
#[derive(Serialize, Debug)]
pub struct MovementType {
    // pub pk: i32,
    pub name: String,
    pub level: i32,
    pub parent: Option<ParentMovementType>,
}

impl From<finances_db::models::MovementType> for MovementType {
    fn from(value: finances_db::models::MovementType) -> Self {
        Self {
            name: value.name,
            level: value.level,
            parent: value.parent_id.map(ParentMovementType::Id),
        }
    }
}
