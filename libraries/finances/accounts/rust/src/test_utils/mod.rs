use diesel::r2d2::{ConnectionManager, Pool};

/// Returns a connection pool to a Postgres database using `POSTGRES_URL` envvar for the connection string
pub fn establish_connection() -> Pool<ConnectionManager<diesel::pg::PgConnection>> {
    let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL environment variable is not set.");

    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(&database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Error creating DB connection pool")
}
