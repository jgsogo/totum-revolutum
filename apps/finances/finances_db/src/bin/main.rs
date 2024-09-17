use clap::Parser;
use diesel::prelude::*;
use finances_db::{
    establish_connection,
    models::{Account, AccountHolder, AccountType},
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env)]
    database_url: String, // --database-url or DATABASE_URL env var
}

pub fn main() {
    let args = Args::parse();

    println!("DB connection string: {}!", args.database_url);
    let mut conn = establish_connection(&args.database_url);

    let results = Account::all_with_holder_and_type()
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for (account, holder, atype) in results {
        println!(
            "{:3} - {:2} - {:30} - {:20} - {}",
            account.id, holder.owner, holder.name, atype.name, account.name
        );
    }
}
