use diesel::prelude::*;

use super::{AccountHolder, AccountType};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::data_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

pub type Opened = diesel::dsl::Or<
    diesel::dsl::IsNull<crate::schema::data_account::close>,
    diesel::dsl::GtEq<crate::schema::data_account::close, diesel::dsl::today>,
>;

pub type Mine = diesel::dsl::Eq<crate::schema::data_accountholder::owner, i32>;

pub type CheckingAccount<'a> = diesel::dsl::Or<
    diesel::dsl::Eq<crate::schema::data_accounttype::name, &'a str>,
    diesel::dsl::Eq<crate::schema::data_accounttype::name, &'a str>,
>;

impl Account {
    pub fn all_with_holder_and_type() -> AllWithHolderAndType {
        use crate::schema::*;
        data_account::table
            .inner_join(data_accountholder::table)
            .inner_join(data_accounttype::table)
    }

    pub fn opened() -> Opened {
        use crate::schema::*;
        data_account::close
            .is_null()
            .or(data_account::close.ge(diesel::dsl::today))
    }

    pub fn mine() -> Mine {
        use crate::schema::*;
        data_accountholder::owner.eq(0)
    }

    pub fn checking_account<'a>() -> CheckingAccount<'a> {
        use crate::schema::*;
        data_accounttype::name
            .eq("Cuenta corriente")
            .or(data_accounttype::name.eq("Metálico"))
    }

    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn investment() -> _ {
        crate::schema::data_accounttype::name
            .eq("Fondo de inversión")
            .or(crate::schema::data_accounttype::name.eq("Acciones"))
            .or(crate::schema::data_accounttype::name.eq("Vivienda"))
            .or(crate::schema::data_accounttype::name.eq("Depósito"))
    }

    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn retirement() -> _ {
        crate::schema::data_accounttype::name.eq("Plan de pensiones")
    }
}

impl Account {
    //     pub fn holder(&self) -> AccountHolder {
    //         todo!("Return the AccountHolder given an Account")
    //     }

    //     pub fn r#type(&self) -> AccountType {
    //         todo!("Return the AccountType given an Account")
    //     }

    pub fn last_snapshot(
        &self,
    ) -> diesel::dsl::FindBy<crate::schema::data_snapshot::table, crate::schema::data_snapshot::account_id, i32> {
        use crate::schema::*;
        data_snapshot::table.filter(data_snapshot::account_id.eq(self.id))
    }

    //     pub fn position(&self) -> BigDecimal {
    //         todo!("Return the position NOW")
    //     }

    //     // pub fn get_position(&self, date: Date) -> BigDecimal {
    //     //     todo!("Return the position at a given DATE")
    //     // }
}
