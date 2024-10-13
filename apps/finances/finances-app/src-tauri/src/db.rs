use crate::types::ConnectionType;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection(database_url: &str) -> Pool<ConnectionManager<ConnectionType>> {
    let manager = ConnectionManager::<ConnectionType>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create DB pool")
}
