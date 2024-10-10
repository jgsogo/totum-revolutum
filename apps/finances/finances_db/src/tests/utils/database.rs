use crate::models::account_type::{ACCIONES, CUENTA_CORRIENTE, DEPOSITO, PLAN_PENSIONES};
use crate::models::movement_type::{
    ACCIONES as ACCIONES_MOVTYPE, AYUDAS_SUBVENCIONES, CHALET_ATYKA, IMPUESTOS, LOPE_DE_HARO, MOVIMIENTO_EFECTIVO,
    RENDIMIENTOS_BIENES_INMUEBLES, RENDIMIENTOS_CAPITAL, RENDIMIENTOS_TRABAJO,
};

use crate::types::NumericType;
use anyhow::Result;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use tempfile::NamedTempFile;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// An object containing a temporary (file-based) SQLite database and its connection
pub struct SqliteTestDatabase {
    _file: NamedTempFile,
    pub conn: SqliteConnection,
}

impl SqliteTestDatabase {
    /// Creates a new [`SqliteTestDatabase`] instance using a temporary file
    pub fn new() -> Self {
        let dbfile = NamedTempFile::new().expect("Failed to create temporary file");
        let dbfile_str = dbfile.path().to_str().unwrap();
        let mut conn = SqliteConnection::establish(dbfile_str).expect("Failed to establish connection to database");
        diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
            .execute(&mut conn)
            .unwrap();

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        Self { _file: dbfile, conn }
    }

    pub fn populate_account_holders(&mut self) -> Result<()> {
        use crate::schema::data_accountholder::dsl::*;
        diesel::insert_into(data_accountholder)
            .values(&vec![
                (id.eq(0), name.eq("holder0"), owner.eq(0)),
                (id.eq(1), name.eq("holder1"), owner.eq(0)),
                (id.eq(2), name.eq("holder2"), owner.eq(0)),
                (id.eq(3), name.eq("holder3"), owner.eq(1)),
                (id.eq(4), name.eq("holder4"), owner.eq(1)),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn populate_account_types(&mut self) -> Result<()> {
        use crate::schema::data_accounttype::dsl::*;
        diesel::insert_into(data_accounttype)
            .values(&vec![
                (id.eq(0), name.eq(CUENTA_CORRIENTE)),
                (id.eq(1), name.eq(DEPOSITO)),
                (id.eq(2), name.eq(ACCIONES)),
                (id.eq(3), name.eq(PLAN_PENSIONES)),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn populate_accounts(&mut self) -> Result<()> {
        use crate::schema::data_account::dsl::*;
        diesel::insert_into(data_account)
            .values(&vec![
                (
                    id.eq(0),
                    identifier.eq("1234"),
                    name.eq("Gastos compartidos"),
                    is_numerable.eq(false),
                    ccy.eq("EUR"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    close.eq(None),
                    holder_id.eq(0),
                    type_id.eq(0),
                ),
                (
                    id.eq(1),
                    identifier.eq("2345"),
                    name.eq("IBM"),
                    is_numerable.eq(true),
                    ccy.eq("USD"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
                    close.eq(None),
                    holder_id.eq(1),
                    type_id.eq(2),
                ),
                (
                    id.eq(2),
                    identifier.eq("345 - closed"),
                    name.eq("Netflix"),
                    is_numerable.eq(true),
                    ccy.eq("USD"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
                    close.eq(Some(chrono::NaiveDate::from_ymd_opt(2024, 10, 8).unwrap())),
                    holder_id.eq(2),
                    type_id.eq(2),
                ),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    /// Populates some snapshots for the given account
    pub fn populate_snapshots(&mut self, account_pk: i32) -> Result<()> {
        use crate::schema::data_snapshot::dsl::*;
        diesel::insert_into(data_snapshot)
            .values(&vec![
                (
                    amount.eq::<NumericType>(0.into()),
                    quantity.eq::<Option<i32>>(None),
                    unit_value.eq::<Option<NumericType>>(None),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    account_id.eq(account_pk),
                ),
                (
                    amount.eq::<NumericType>(1.into()),
                    quantity.eq::<Option<i32>>(None),
                    unit_value.eq::<Option<NumericType>>(None),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()),
                    account_id.eq(account_pk),
                ),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn populate_movement_types(&mut self) -> Result<()> {
        use crate::schema::data_movementtype::dsl::*;
        diesel::insert_into(data_movementtype)
            .values(&vec![
                (
                    id.eq(0),
                    name.eq(RENDIMIENTOS_BIENES_INMUEBLES),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(1),
                    name.eq(RENDIMIENTOS_TRABAJO),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(2),
                    name.eq(LOPE_DE_HARO),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(3),
                    name.eq(RENDIMIENTOS_CAPITAL),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(4),
                    name.eq(MOVIMIENTO_EFECTIVO),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(5),
                    name.eq(IMPUESTOS),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(6),
                    name.eq(ACCIONES_MOVTYPE),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(7),
                    name.eq(AYUDAS_SUBVENCIONES),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
                (
                    id.eq(8),
                    name.eq(CHALET_ATYKA),
                    level.eq(1),
                    parent_id.eq::<Option<i32>>(None),
                ),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn populate_transfers(&mut self) -> Result<()> {
        use crate::schema::data_transfer::dsl::*;
        diesel::insert_into(data_transfer)
            .values(&vec![
                (id.eq(0), description.eq("transfer0")),
                (id.eq(1), description.eq("transfer1")),
                (id.eq(2), description.eq("transfer2")),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    /// Populates some movements for the given account
    pub fn populate_movements(&mut self, account_pk: i32) -> Result<()> {
        use crate::schema::data_movement::dsl::*;
        diesel::insert_into(data_movement)
            .values(&vec![
                (
                    amount.eq::<NumericType>(0.into()),
                    quantity.eq::<Option<i32>>(None),
                    unit_value.eq::<Option<NumericType>>(None),
                    direction.eq(0),
                    date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                    account_id.eq(account_pk),
                    fx_id.eq::<Option<i32>>(None),
                    transfer_id.eq::<i32>(0),
                    type_id.eq::<i32>(0),
                ),
                (
                    amount.eq::<NumericType>(0.into()),
                    quantity.eq::<Option<i32>>(None),
                    unit_value.eq::<Option<NumericType>>(None),
                    direction.eq(0),
                    date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
                    date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 7).unwrap()),
                    account_id.eq(account_pk),
                    fx_id.eq::<Option<i32>>(None),
                    transfer_id.eq::<i32>(0),
                    type_id.eq::<i32>(0),
                ),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }
}
