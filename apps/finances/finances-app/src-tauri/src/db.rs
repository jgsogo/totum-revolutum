use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create DB pool")
}
