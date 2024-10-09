use diesel::prelude::*;

use super::{AccountHolder, AccountType};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::data_account)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(AccountHolder, foreign_key = holder_id))]
#[diesel(belongs_to(AccountType, foreign_key = type_id))]
pub struct Account {
    pub id: i32,
    pub identifier: Option<String>,
    pub name: String,
    pub is_numerable: bool,
    pub ccy: String,
    pub open: chrono::NaiveDate,
    pub close: Option<chrono::NaiveDate>,
    pub holder_id: i32,
    pub type_id: i32,
}

pub type AllWithHolderAndType = diesel::dsl::InnerJoin<
    diesel::dsl::InnerJoin<crate::schema::data_account::table, crate::schema::data_accountholder::table>,
    crate::schema::data_accounttype::table,
>;

impl Account {
    /// Returns (a query to) all the [`Account`]s together with their [`AccountHolder`] and [`AccountType`]
    pub fn all_with_holder_and_type() -> AllWithHolderAndType {
        crate::schema::data_account::table
            .inner_join(crate::schema::data_accountholder::table)
            .inner_join(crate::schema::data_accounttype::table)
    }

    /// Returns (a query to) all the ([`Account`], [`AccountHolder`], [`AccountType`]) for a given account primary-key
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn get_with_holder_and_type(pk: i32) -> _ {
        let all: AllWithHolderAndType = Account::all_with_holder_and_type();
        all.filter(crate::schema::data_account::id.eq(pk))
    }

    /// Returns a query fragment to filter all the [`Account`]s that are opened as of today
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn opened() -> _ {
        crate::schema::data_account::close
            .is_null()
            .or(crate::schema::data_account::close.ge(diesel::dsl::today))
    }

    /// Returns a query fragment to filter all the [`Account`]s that are owned by ME
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn mine() -> _ {
        crate::schema::data_accountholder::owner.eq(0i32)
    }

    /// Returns a query fragment to filter all the [`Account`]s whose [`AccountType`] is either
    /// "Cuenta corriente" or "Metálico"
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn checking_account() -> _ {
        crate::schema::data_accounttype::name
            .eq("Cuenta corriente")
            .or(crate::schema::data_accounttype::name.eq("Metálico"))
    }

    /// Returns a query fragment to filter all the [`Account`]s whose [`AccountType`] is either
    /// "Fondo de inversión", "Acciones", "Vivienda" or "Depósito"
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn investment() -> _ {
        crate::schema::data_accounttype::name
            .eq("Fondo de inversión")
            .or(crate::schema::data_accounttype::name.eq("Acciones"))
            .or(crate::schema::data_accounttype::name.eq("Vivienda"))
            .or(crate::schema::data_accounttype::name.eq("Depósito"))
    }

    /// Returns a query fragment to filter all the [`Account`]s whose [`AccountType`] is
    /// "Plan de pensiones"
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn retirement() -> _ {
        crate::schema::data_accounttype::name.eq("Plan de pensiones")
    }
}

impl Account {
    /// Returns (a query to) the latest [`super::Snapshot`] for this account
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn last_snapshot(&self) -> _ {
        let id: i32 = self.id;
        crate::schema::data_snapshot::table.filter(crate::schema::data_snapshot::account_id.eq(id))
    }

    //     pub fn position(&self) -> BigDecimal {
    //         todo!("Return the position NOW")
    //     }

    //     // pub fn get_position(&self, date: Date) -> BigDecimal {
    //     //     todo!("Return the position at a given DATE")
    //     // }
}
