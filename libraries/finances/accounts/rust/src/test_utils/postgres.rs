use crate::models::{AccountType, MovementType};
use crate::sql::filters::{acounttype_by_unique_name, movementtype_by_unique_name};
use crate::types::NumericType;
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct TestDatabase {
    pub pool: Pool<ConnectionManager<diesel::pg::PgConnection>>,
}

pub fn establish_connection(database_url: &str) -> Pool<ConnectionManager<diesel::pg::PgConnection>> {
    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Error creating DB connection pool")
}

impl TestDatabase {
    /// Creates a new [`TestDatabase`]
    pub fn new() -> Self {
        let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL environment variable is not set.");
        let pool = establish_connection(&database_url);
        Self { pool }
    }

    // pub fn populate_account_holders(&mut self) -> Result<()> {
    //     use crate::schema::data_accountholder::dsl::*;
    //     diesel::insert_into(data_accountholder)
    //         .values(&vec![
    //             (id.eq(0), name.eq("holder0"), owner.eq(0)),
    //             (id.eq(1), name.eq("holder1"), owner.eq(0)),
    //             (id.eq(2), name.eq("holder2"), owner.eq(0)),
    //             (id.eq(3), name.eq("holder3"), owner.eq(1)),
    //             (id.eq(4), name.eq("holder4"), owner.eq(1)),
    //         ])
    //         .execute(&mut self.conn)?;
    //     Ok(())
    // }

    pub fn populate_custodians(&mut self) -> Result<()> {
        use crate::schema::finances_accounts_custodian::dsl::*;
        diesel::insert_into(finances_accounts_custodian)
            .values(&vec![
                (id.eq(0), name.eq("custodian0"), country.eq("es")),
                (id.eq(1), name.eq("custodian1"), country.eq("us")),
                (id.eq(2), name.eq("custodian2"), country.eq("nl")),
            ])
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    pub fn populate_accounts(&mut self) -> Result<()> {
        use crate::schema::finances_accounts_account::dsl::*;

        let mut conn = self.pool.get()?;
        let assets = AccountType::all()
            .filter(acounttype_by_unique_name(crate::constants::accounttype::ASSETS))
            .select(AccountType::as_select())
            .first(&mut conn)?;
        let assets_current = AccountType::all()
            .filter(acounttype_by_unique_name(crate::constants::accounttype::ASSETS_CURRENT))
            .select(AccountType::as_select())
            .first(&mut conn)?;

        diesel::insert_into(finances_accounts_account)
            .values(&vec![
                (
                    id.eq(0),
                    name.eq("Gastos compartidos"),
                    description.eq(None::<String>),
                    identifier.eq("1234"),
                    ccy.eq("EUR"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    close.eq(None),
                    type_id.eq(assets_current.id),
                    custodian_id.eq(0),
                    is_numerable.eq(false),
                ),
                (
                    id.eq(1),
                    name.eq("IBM"),
                    description.eq(None::<String>),
                    identifier.eq("2345"),
                    ccy.eq("USD"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
                    close.eq(None),
                    type_id.eq(assets.id),
                    custodian_id.eq(1),
                    is_numerable.eq(true),
                ),
                (
                    id.eq(2),
                    name.eq("Netflix"),
                    description.eq(None::<String>),
                    identifier.eq("345 - closed"),
                    ccy.eq("USD"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
                    close.eq(Some(chrono::NaiveDate::from_ymd_opt(2024, 10, 8).unwrap())),
                    type_id.eq(assets.id),
                    custodian_id.eq(2),
                    is_numerable.eq(true),
                ),
            ])
            .execute(&mut conn)?;
        Ok(())
    }

    /// Populates some snapshots for the given account
    pub fn populate_snapshots(&mut self, account_pk: i64) -> Result<()> {
        use crate::schema::finances_accounts_snapshot::dsl::*;
        let mut conn = self.pool.get()?;
        diesel::insert_into(finances_accounts_snapshot)
            .values(&vec![
                (
                    amount.eq::<NumericType>(0.into()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    account_id.eq(account_pk),
                ),
                (
                    amount.eq::<NumericType>(1.into()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
                    account_id.eq(account_pk),
                ),
            ])
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn populate_transactions(&mut self) -> Result<()> {
        use crate::schema::finances_accounts_transaction::dsl::*;
        diesel::insert_into(finances_accounts_transaction)
            .values(&vec![
                (id.eq(0), name.eq("transaction0")),
                (id.eq(1), name.eq("transaction1")),
                (id.eq(2), name.eq("transaction2")),
            ])
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    /// Populates some movements for the given account
    pub fn populate_movements(&mut self, account_pk: i64) -> Result<Vec<i64>> {
        self.populate_fx(account_pk * 10)?;

        let mut conn = self.pool.get()?;
        let expense = MovementType::all()
            .filter(movementtype_by_unique_name(crate::constants::movementtype::EXPENSE))
            .select(MovementType::as_select())
            .first(&mut conn)?;

        use crate::schema::finances_accounts_movement::dsl::*;
        let results: Vec<i64> = diesel::insert_into(finances_accounts_movement)
            .values(&vec![
                (
                    amount.eq::<NumericType>(0.into()),
                    direction.eq(0),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    account_id.eq(account_pk),
                    fx_id.eq::<Option<i64>>(Some(account_pk * 10)),
                    transaction_id.eq::<i64>(0),
                    type_id.eq::<i64>(expense.id),
                ),
                (
                    amount.eq::<NumericType>(0.into()),
                    direction.eq(0),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
                    account_id.eq(account_pk),
                    fx_id.eq::<Option<i64>>(Some(account_pk * 10 + 1)),
                    transaction_id.eq::<i64>(0),
                    type_id.eq::<i64>(expense.id),
                ),
            ])
            .returning(id)
            .get_results(&mut conn)?;
        Ok(results)
    }

    /// Populates some FX values
    pub fn populate_fx(&mut self, start_id: i64) -> Result<()> {
        use crate::schema::finances_accounts_fx::dsl::*;
        diesel::insert_into(finances_accounts_fx)
            .values(&vec![
                (
                    id.eq(start_id),
                    foreign.eq("USD"),
                    local.eq("EUR"),
                    rate.eq::<NumericType>(1.into()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                ),
                (
                    id.eq(start_id + 1),
                    foreign.eq("USD"),
                    local.eq("EUR"),
                    rate.eq::<NumericType>(2.into()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
                ),
                (
                    id.eq(start_id + 2),
                    foreign.eq("USD"),
                    local.eq("EUR"),
                    rate.eq::<NumericType>(3.into()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
                ),
                (
                    id.eq(start_id + 3),
                    foreign.eq("USD"),
                    local.eq("EUR"),
                    rate.eq::<NumericType>(4.into()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 9).unwrap()),
                ),
            ])
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }
}
