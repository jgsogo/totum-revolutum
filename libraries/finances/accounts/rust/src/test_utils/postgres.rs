use tracing::error;

use crate::models::AccountType;
use anyhow::Result;
use diesel::prelude::*;

pub struct TestDatabase {
    pub conn: PgConnection,
}

pub fn establish_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
    match PgConnection::establish(&database_url) {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("Could not connect to PostgreSQL.");
            error!("Error connecting to {}", database_url);
            Err(e)
        }
    }
}

impl TestDatabase {
    /// Creates a new [`TestDatabase`]
    pub fn new() -> Self {
        let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL environment variable is not set.");
        let conn = establish_connection(&database_url).unwrap();

        Self { conn }
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

    // pub fn populate_account_types(&mut self) -> Result<()> {
    //     use crate::schema::data_accounttype::dsl::*;
    //     diesel::insert_into(data_accounttype)
    //         .values(&vec![
    //             (id.eq(0), name.eq(CUENTA_CORRIENTE)),
    //             (id.eq(1), name.eq(DEPOSITO)),
    //             (id.eq(2), name.eq(ACCIONES)),
    //             (id.eq(3), name.eq(PLAN_PENSIONES)),
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
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn populate_accounts(&mut self) -> Result<()> {
        use crate::schema::finances_accounts_account::dsl::*;

        let assets = AccountType::get_by_unique_name(crate::constants::accounttype::ASSETS)
            .select(AccountType::as_select())
            .first(&mut self.conn)?;
        let assets_current = AccountType::get_by_unique_name(crate::constants::accounttype::ASSETS_CURRENT)
            .select(AccountType::as_select())
            .first(&mut self.conn)?;

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
            .execute(&mut self.conn)?;
        Ok(())
    }

    // /// Populates some snapshots for the given account
    // pub fn populate_snapshots(&mut self, account_pk: i32) -> Result<()> {
    //     use crate::schema::data_snapshot::dsl::*;
    //     diesel::insert_into(data_snapshot)
    //         .values(&vec![
    //             (
    //                 amount.eq::<NumericType>(0.into()),
    //                 quantity.eq::<Option<i32>>(None),
    //                 unit_value.eq::<Option<NumericType>>(None),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
    //                 account_id.eq(account_pk),
    //             ),
    //             (
    //                 amount.eq::<NumericType>(1.into()),
    //                 quantity.eq::<Option<i32>>(None),
    //                 unit_value.eq::<Option<NumericType>>(None),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
    //                 account_id.eq(account_pk),
    //             ),
    //         ])
    //         .execute(&mut self.conn)?;
    //     Ok(())
    // }

    // pub fn populate_movement_types(&mut self) -> Result<()> {
    //     use crate::schema::data_movementtype::dsl::*;
    //     diesel::insert_into(data_movementtype)
    //         .values(&vec![
    //             (
    //                 id.eq(0),
    //                 name.eq(RENDIMIENTOS_BIENES_INMUEBLES),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(1),
    //                 name.eq(RENDIMIENTOS_TRABAJO),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(2),
    //                 name.eq(LOPE_DE_HARO),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(3),
    //                 name.eq(RENDIMIENTOS_CAPITAL),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(4),
    //                 name.eq(MOVIMIENTO_EFECTIVO),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(5),
    //                 name.eq(IMPUESTOS),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(6),
    //                 name.eq(ACCIONES_MOVTYPE),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(7),
    //                 name.eq(AYUDAS_SUBVENCIONES),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //             (
    //                 id.eq(8),
    //                 name.eq(CHALET_ATYKA),
    //                 level.eq(1),
    //                 parent_id.eq::<Option<i32>>(None),
    //             ),
    //         ])
    //         .execute(&mut self.conn)?;
    //     Ok(())
    // }

    // pub fn populate_transfers(&mut self) -> Result<()> {
    //     use crate::schema::data_transfer::dsl::*;
    //     diesel::insert_into(data_transfer)
    //         .values(&vec![
    //             (id.eq(0), description.eq("transfer0")),
    //             (id.eq(1), description.eq("transfer1")),
    //             (id.eq(2), description.eq("transfer2")),
    //         ])
    //         .execute(&mut self.conn)?;
    //     Ok(())
    // }

    // /// Populates some movements for the given account
    // pub fn populate_movements(&mut self, account_pk: i32) -> Result<()> {
    //     self.populate_fx(account_pk * 10)?;

    //     use crate::schema::data_movement::dsl::*;
    //     diesel::insert_into(data_movement)
    //         .values(&vec![
    //             (
    //                 amount.eq::<NumericType>(0.into()),
    //                 quantity.eq::<Option<i32>>(None),
    //                 unit_value.eq::<Option<NumericType>>(None),
    //                 direction.eq(0),
    //                 date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
    //                 account_id.eq(account_pk),
    //                 fx_id.eq::<Option<i32>>(Some(account_pk * 10)),
    //                 transfer_id.eq::<i32>(0),
    //                 type_id.eq::<i32>(0),
    //             ),
    //             (
    //                 amount.eq::<NumericType>(0.into()),
    //                 quantity.eq::<Option<i32>>(None),
    //                 unit_value.eq::<Option<NumericType>>(None),
    //                 direction.eq(0),
    //                 date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
    //                 account_id.eq(account_pk),
    //                 fx_id.eq::<Option<i32>>(Some(account_pk * 10 + 1)),
    //                 transfer_id.eq::<i32>(0),
    //                 type_id.eq::<i32>(0),
    //             ),
    //         ])
    //         .execute(&mut self.conn)?;
    //     Ok(())
    // }

    // /// Populates some FX values
    // pub fn populate_fx(&mut self, start_id: i32) -> Result<()> {
    //     use crate::schema::data_fx::dsl::*;
    //     diesel::insert_into(data_fx)
    //         .values(&vec![
    //             (
    //                 id.eq(start_id),
    //                 foreign.eq("USD"),
    //                 local.eq("EUR"),
    //                 rate.eq::<NumericType>(1.into()),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
    //             ),
    //             (
    //                 id.eq(start_id + 1),
    //                 foreign.eq("USD"),
    //                 local.eq("EUR"),
    //                 rate.eq::<NumericType>(2.into()),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
    //             ),
    //             (
    //                 id.eq(start_id + 2),
    //                 foreign.eq("USD"),
    //                 local.eq("EUR"),
    //                 rate.eq::<NumericType>(3.into()),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
    //             ),
    //             (
    //                 id.eq(start_id + 3),
    //                 foreign.eq("USD"),
    //                 local.eq("EUR"),
    //                 rate.eq::<NumericType>(4.into()),
    //                 date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 9).unwrap()),
    //             ),
    //         ])
    //         .execute(&mut self.conn)?;
    //     Ok(())
    // }
}
