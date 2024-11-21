use diesel::prelude::*;

use finances_accounts::types::NumericType;

use finances_accounts::models::Movement;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_investments_movementnumerable)]
#[diesel(primary_key(movement_ptr_id))]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Movement, foreign_key = movement_ptr_id))]
pub struct MovementNumerable {
    pub movement_ptr_id: i64,
    pub quantity: NumericType,
    pub unit_value: NumericType,
}

// impl MovementNumerable {
//     /// Returns (a query to) all the `MovementNumerable`s for a given account primary-key
//     #[diesel::dsl::auto_type(no_type_alias)]
//     pub fn all_with_related_data(account_pk: i64) -> _ {
//         crate::schema::finances_investments_movementnumerable::table
//             .inner_join(finances_accounts::schema::finances_accounts_movement::table)
//             // .inner_join(crate::schema::finances_accounts_fx::table) // FIXME: I cannot 'inner_join' a nullable FK
//             // .inner_join(finances_accounts::schema::finances_accounts_transaction::table)
//             // .inner_join(finances_accounts::schema::finances_accounts_movementtype::table)
//             // .filter(finances_accounts::schema::finances_accounts_movement::account_id.eq(account_pk))
//             // .order((finances_accounts::schema::finances_accounts_movement::date_value.desc(),))
//     }
// }
