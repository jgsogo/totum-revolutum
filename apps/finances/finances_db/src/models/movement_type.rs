use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_movementtype)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MovementType {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub parent_id: Option<i32>,
}
