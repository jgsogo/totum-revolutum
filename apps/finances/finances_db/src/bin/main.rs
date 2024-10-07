use clap::Parser;
use diesel::prelude::*;
use finances_db::{
    establish_connection,
    models::{Account, AccountHolder, AccountType, Snapshot},
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env)]
    database_url: String, // --database-url or DATABASE_URL env var
}

pub fn main() {
    let args = Args::parse();

    println!("DB connection string: {}!", args.database_url);
    let pool = establish_connection(&args.database_url);
    let mut conn = pool.get().expect("Get a connection from the Pool");

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
        let snapshot: Option<Snapshot> = account
            .last_snapshot()
            .first(&mut conn)
            .optional()
            .expect("Error returning the last snapshot"); // FIXME: This is n+1 query
        let snapshot_amount = match snapshot {
            Some(snapshot) => snapshot.amount,
            None => 0f64,
        };
        println!(
            "{:3} - {:2} - {:30} - {:20} - {:40} - {:9.2}",
            account.id, holder.owner, holder.name, atype.name, account.name, snapshot_amount
        );
    }
}
