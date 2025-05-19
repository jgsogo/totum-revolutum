use diesel::prelude::*;

use super::{AccountHolder, AccountType, Custodian};
use crate::fields::MovementDirection;
use crate::models::{Movement, MovementType, Snapshot, Transaction};
use crate::sql::filters::{account_by_pk, movement_filter_account_by_pk, snapshot_filter_account_by_pk};
use crate::sql::filters::{account_closed, account_opened};
use crate::types::NumericType;
use bigdecimal::Zero;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_account)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(AccountType, foreign_key = type_id))]
#[diesel(belongs_to(Custodian, foreign_key = custodian_id))]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub identifier: Option<String>,
    pub ccy: String,
    pub open: chrono::NaiveDate,
    pub close: Option<chrono::NaiveDate>,
    pub type_id: i64,
    pub custodian_id: i64,
    pub is_numerable: bool,
}

impl Account {
    /// Returns (a query to) all the [`Account`]s (opened and closed)
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_account::table
    }

    /// Returns (a query to) all the [`Account`]s (only opened ones)
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_opened() -> _ {
        crate::schema::finances_accounts_account::table.filter(account_opened())
    }

    /// Returns (a query to) all the [`Account`]s that are closed
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_closed() -> _ {
        crate::schema::finances_accounts_account::table.filter(account_closed())
    }
}

impl Account {
    pub fn from_pk(pk: i64, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        Self::all()
            .filter(account_by_pk(pk))
            .select(Account::as_select())
            .first::<Account>(conn)
    }

    /// Returns the list of [`AccountHolder`]a that are related to this account together with a mark
    /// wether this holder owns money in the account or not.
    pub fn holders_for_pk(
        pk: i64,
        conn: &mut PgConnection,
    ) -> Result<Vec<(AccountHolder, bool)>, diesel::result::Error> {
        Self::all()
            .inner_join(
                crate::schema::finances_accounts_accountholderrole::table
                    .inner_join(crate::schema::finances_accounts_accountholder::table),
            )
            .select((
                AccountHolder::as_select(),
                crate::schema::finances_accounts_accountholderrole::owns_money,
            ))
            .filter(account_by_pk(pk))
            .load::<(AccountHolder, bool)>(conn)
    }

    pub fn details_for_pk(
        pk: i64,
        conn: &mut PgConnection,
    ) -> Result<(Account, Custodian, AccountType), diesel::result::Error> {
        Self::all()
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .filter(account_by_pk(pk))
            .first::<(Account, Custodian, AccountType)>(conn)
    }

    pub fn details(&self, conn: &mut PgConnection) -> Result<(Custodian, AccountType), diesel::result::Error> {
        Self::all()
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .select((
                // Account::as_select(),
                Custodian::as_select(),
                AccountType::as_select(),
            ))
            .filter(account_by_pk(self.id))
            .first::<(Custodian, AccountType)>(conn)
    }

    pub fn latest_snapshot_for_pk(pk: i64, conn: &mut PgConnection) -> Result<Option<Snapshot>, diesel::result::Error> {
        Snapshot::all()
            .filter(snapshot_filter_account_by_pk(pk))
            .first(conn)
            .optional()
    }
    pub fn latest_snapshot(&self, conn: &mut PgConnection) -> Result<Option<Snapshot>, diesel::result::Error> {
        Self::latest_snapshot_for_pk(self.id, conn)
    }

    pub fn snapshots_for_pk(pk: i64, conn: &mut PgConnection) -> Result<Vec<Snapshot>, diesel::result::Error> {
        Snapshot::all().filter(snapshot_filter_account_by_pk(pk)).load(conn)
    }

    pub fn snapshots(&self, conn: &mut PgConnection) -> Result<Vec<Snapshot>, diesel::result::Error> {
        Self::snapshots_for_pk(self.id, conn)
    }

    pub fn movements_for_pk(
        pk: i64,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Movement, Transaction, MovementType)>, diesel::result::Error> {
        Movement::all()
            .filter(movement_filter_account_by_pk(pk))
            .inner_join(Transaction::all())
            .inner_join(MovementType::all())
            .select((
                Movement::as_select(),
                Transaction::as_select(),
                MovementType::as_select(),
            ))
            .load::<(Movement, Transaction, MovementType)>(conn)
    }

    pub fn movements(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Movement, Transaction, MovementType)>, diesel::result::Error> {
        Self::movements_for_pk(self.id, conn)
    }

    pub fn position_for_pk(
        pk: i64,
        conn: &mut PgConnection,
        date: &chrono::NaiveDate,
    ) -> Result<NumericType, diesel::result::Error> {
        // Get latest snapshot for the given date
        let snapshot: Option<Snapshot> = Snapshot::all()
            .filter(snapshot_filter_account_by_pk(pk))
            .filter(crate::schema::finances_accounts_snapshot::date_value.le(date))
            .first(conn)
            .optional()?;

        // Get all movements between the closest snapshot and the requested date
        let movs: Vec<(NumericType, MovementDirection)> = match snapshot {
            Some(ref snapshot) => {
                Movement::all()
                    .filter(movement_filter_account_by_pk(pk))
                    .filter(crate::schema::finances_accounts_movement::date_value.gt(snapshot.date_value)) // We consider that the snapshot is taken EOD
                    .filter(crate::schema::finances_accounts_movement::date_value.le(date))
                    .select((
                        crate::schema::finances_accounts_movement::amount,
                        crate::schema::finances_accounts_movement::direction,
                    ))
                    .load::<(NumericType, MovementDirection)>(conn)?
            }
            None => Movement::all()
                .filter(movement_filter_account_by_pk(pk))
                .filter(crate::schema::finances_accounts_movement::date_value.le(date))
                .select((
                    crate::schema::finances_accounts_movement::amount,
                    crate::schema::finances_accounts_movement::direction,
                ))
                .load::<(NumericType, MovementDirection)>(conn)?,
        };

        let initial_position: NumericType = snapshot.map_or(NumericType::zero(), |v| v.amount);
        let position = movs.into_iter().fold(initial_position, |total, v| {
            let (amount, direction) = v;
            match direction {
                MovementDirection::In => total + amount,
                MovementDirection::Out => total - amount,
            }
        });
        Ok(position)
    }

    pub fn position(
        &self,
        conn: &mut PgConnection,
        date: &chrono::NaiveDate,
    ) -> Result<NumericType, diesel::result::Error> {
        Self::position_for_pk(self.id, conn, date)
    }
}
