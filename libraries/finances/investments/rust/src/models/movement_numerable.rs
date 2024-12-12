use diesel::prelude::*;

use finances_accounts::types::NumericType;

use finances_accounts::models::{Movement, NewMovement};

use diesel::deserialize::Result;
use diesel::row::NamedRow;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, QueryableByName)]
#[diesel(table_name = crate::schema::finances_investments_movementnumerable)]
#[diesel(primary_key(movement_ptr_id))]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Movement, foreign_key = movement_ptr_id))]
struct _MovementNumerable {
    movement_ptr_id: i64,
    #[allow(dead_code)]
    quantity: NumericType,
    #[allow(dead_code)]
    unit_value: NumericType,
}

pub struct MovementNumerable {
    #[allow(dead_code)]
    movement: Movement,
    #[allow(dead_code)]
    movement_numerable: _MovementNumerable,
}

impl QueryableByName<finances_accounts::types::BackendType> for MovementNumerable
where
    Self: Sized,
{
    fn build<'a>(row: &impl NamedRow<'a, finances_accounts::types::BackendType>) -> Result<Self> {
        let movement = <Movement as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        let movement_numerable =
            <_MovementNumerable as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        Ok(MovementNumerable {
            movement,
            movement_numerable,
        })
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_investments_movementnumerable)]
pub(crate) struct _NewMovementNumerable<'a> {
    pub(crate) movement_ptr_id: &'a i64,
    pub(crate) quantity: &'a NumericType,
    pub(crate) unit_value: &'a NumericType,
}

pub struct NewMovementNumerable<'a> {
    pub new_movement: &'a NewMovement<'a>,
    pub quantity: &'a NumericType,
    pub unit_value: &'a NumericType,
}
