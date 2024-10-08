use anyhow::Result;
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};
use tempfile::NamedTempFile;

pub struct TestDatabase {
    _file: NamedTempFile,
    pub conn: SqliteConnection,
}

impl TestDatabase {
    pub fn new(file: NamedTempFile, conn: SqliteConnection) -> Self {
        Self { _file: file, conn }
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
                (id.eq(0), name.eq("Cuenta Corriente")),
                (id.eq(1), name.eq("Depósito")),
                (id.eq(2), name.eq("Acciones")),
                (id.eq(3), name.eq("Plan de pensiones")),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn populate_accounts(&mut self) -> Result<()> {
        use crate::schema::data_account::dsl::*;
        diesel::insert_into(data_account)
            .values(&vec![
                (
                    // id.eq(0),
                    identifier.eq("1234"),
                    name.eq("Gastos compartidos"),
                    is_numerable.eq(false),
                    ccy.eq("EUR"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
                    close.eq(None),
                    holder_id.eq(0),
                    type_id.eq(0),
                ),
                (
                    // id.eq(0),
                    identifier.eq("2345"),
                    name.eq("IBM"),
                    is_numerable.eq(true),
                    ccy.eq("USD"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
                    close.eq(None),
                    holder_id.eq(0),
                    type_id.eq(2),
                ),
                (
                    // id.eq(0),
                    identifier.eq("345 - closed"),
                    name.eq("Netflix"),
                    is_numerable.eq(true),
                    ccy.eq("USD"),
                    open.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 8).unwrap()),
                    close.eq(Some(chrono::NaiveDate::from_ymd_opt(2024, 10, 8).unwrap())),
                    holder_id.eq(0),
                    type_id.eq(2),
                ),
            ])
            .execute(&mut self.conn)?;
        Ok(())
    }
}
