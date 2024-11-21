use diesel::prelude::*;

use finances_accounts::types::NumericType;

use diesel::query_builder::SqlQuery;
use diesel::sql_query;
use finances_accounts::models::Movement;

use diesel::deserialize::Result;
use diesel::row::NamedRow;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, QueryableByName)]
#[diesel(table_name = crate::schema::finances_investments_movementnumerable)]
#[diesel(primary_key(movement_ptr_id))]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Movement, foreign_key = movement_ptr_id))]
pub struct MovementNumerable {
    pub movement_ptr_id: i64,
    pub quantity: NumericType,
    pub unit_value: NumericType,
}

pub struct MovementNumerableType {
    pub movement: Movement,
    pub movement_numerable: MovementNumerable,
}

impl QueryableByName<finances_accounts::types::BackendType> for MovementNumerableType
where
    Self: Sized,
{
    fn build<'a>(row: &impl NamedRow<'a, finances_accounts::types::BackendType>) -> Result<Self> {
        let movement = <Movement as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        let movement_numerable =
            <MovementNumerable as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        Ok(MovementNumerableType {
            movement,
            movement_numerable,
        })
    }
}

impl MovementNumerable {
    /// Returns (a query to) all the `MovementNumerable`s for a given account primary-key
    pub fn all_with_related_data_raw(_account_pk: i64) -> SqlQuery {
        sql_query(
            r#"
            SELECT *
            FROM finances_accounts_movement
            INNER JOIN
                finances_investments_movementnumerable
            ON
                finances_accounts_movement.id = finances_investments_movementnumerable.movement_ptr_id
            WHERE
                finances_accounts_movement.account_id = $1
        "#,
        )
        // FIXME: Figure out how to bind the account_id here
        // .bind::<diesel::sql_types::Int8, _>(_account_pk)
    }
}
