mod connection;
pub mod models;
pub use connection::establish_connection;
pub mod schema;

pub fn test() {
    println!("Test");
}
