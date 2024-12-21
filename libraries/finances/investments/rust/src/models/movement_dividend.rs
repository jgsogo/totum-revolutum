use diesel::prelude::*;

use finances_accounts::types::NumericType;

use diesel::deserialize::Result;
use diesel::row::NamedRow;
use finances_accounts::models::Movement;
use finances_accounts::models::NewMovement;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, QueryableByName)]
#[diesel(table_name = crate::schema::finances_investments_movementdividend)]
#[diesel(primary_key(movement_ptr_id))]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Movement, foreign_key = movement_ptr_id))]
pub struct _MovementDividend {
    pub movement_ptr_id: i64,
    pub ex_dividend_date: chrono::NaiveDate,
    pub unit_value: NumericType,
}

pub struct MovementDividend {
    pub movement: Movement,
    pub movement_dividend: _MovementDividend,
}

impl QueryableByName<finances_accounts::types::BackendType> for MovementDividend
where
    Self: Sized,
{
    fn build<'a>(row: &impl NamedRow<'a, finances_accounts::types::BackendType>) -> Result<Self> {
        let movement = <Movement as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        let movement_dividend =
            <_MovementDividend as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        Ok(MovementDividend {
            movement,
            movement_dividend,
        })
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_investments_movementdividend)]
pub(crate) struct _NewMovementDividend<'a> {
    pub(crate) movement_ptr_id: &'a i64,
    pub(crate) ex_dividend_date: &'a chrono::NaiveDate,
    pub(crate) unit_value: &'a NumericType,
}

pub struct NewMovementDividend<'a> {
    pub new_movement: &'a NewMovement<'a>,
    pub ex_dividend_date: &'a chrono::NaiveDate,
    pub unit_value: &'a NumericType,
}

impl NewMovementDividend<'_> {
    pub fn insert_into_db(&self, conn: &mut PgConnection) -> std::result::Result<i64, diesel::result::Error> {
        conn.transaction(|conn| {
            let inner_movement_pk = self.new_movement.insert_into_db(conn)?;

            let new_movement_dividend = _NewMovementDividend {
                movement_ptr_id: &inner_movement_pk,
                ex_dividend_date: self.ex_dividend_date,
                unit_value: self.unit_value,
            };
            diesel::insert_into(crate::schema::finances_investments_movementdividend::table)
                .values(&new_movement_dividend)
                .returning(crate::schema::finances_investments_movementdividend::movement_ptr_id)
                .get_result::<i64>(conn)
        })
    }
}
