mod test_models_account;
mod test_models_account_holder;
mod test_models_account_type;
mod test_models_movement;
mod test_models_movement_type;
mod test_models_snapshot;
mod test_models_transaction;
mod test_models_transaction_group;

use crate::test_utils::establish_connection;
use diesel::r2d2::{ConnectionManager, Pool};
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_POOL: Pool<ConnectionManager<diesel::pg::PgConnection>> = { establish_connection() };
}
