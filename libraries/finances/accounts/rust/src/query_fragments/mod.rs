use diesel::prelude::*;

/// Returns a query fragment to filter all the [`Account`]s that are opened as of today (they don't have close date or it is later than today or equal)
#[diesel::dsl::auto_type]
pub fn account_opened() -> _ {
    crate::schema::finances_accounts_account::close
        .is_null()
        .or(crate::schema::finances_accounts_account::close.ge(diesel::dsl::today))
}

/// Returns a query fragment to filter all the [`Account`]s that are closed as of today (their close data is less than today)
#[diesel::dsl::auto_type]
pub fn account_closed() -> _ {
    crate::schema::finances_accounts_account::close
        .is_not_null()
        .and(crate::schema::finances_accounts_account::close.lt(diesel::dsl::today))
}

/// Returns a query fragment to order accounts [`Account`]s. This can be considered the _default_ ordering
#[diesel::dsl::auto_type]
pub fn account_ordered() -> _ {
    crate::schema::finances_accounts_account::open.desc()
}
