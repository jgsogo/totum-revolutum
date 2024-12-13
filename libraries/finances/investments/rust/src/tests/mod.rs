mod test_models_movement_numerable;
mod test_models_snapshot_numerable;

use crate::test_utils::establish_connection;
use diesel::r2d2::{ConnectionManager, Pool};
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_POOL: Pool<ConnectionManager<diesel::pg::PgConnection>> = establish_connection();
}
