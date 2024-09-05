use clap::Parser;
use diesel::prelude::*;
use finances_db::{establish_connection, models::Account};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env)]
    database_url: String, // --database-url or DATABASE_URL env var
}

pub fn main() {
    let args = Args::parse();

    println!("DB connection string: {}!", args.database_url);
    let mut conn = establish_connection(&args.database_url);

    use finances_db::schema::data_account::dsl::*;
    let results = data_account
        .select(Account::as_select())
        .load(&mut conn)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for post in results {
        println!("{:3} - {}", post.id, post.name);
    }
}
