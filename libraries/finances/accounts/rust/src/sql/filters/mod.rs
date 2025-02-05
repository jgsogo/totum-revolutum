use crate::fields::MovementDirection;
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

/// Returns a query fragment to filter [`Account`]s by pk
#[diesel::dsl::auto_type]
pub fn account_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_account::id.eq(pk)
}

/// Returns a query fragment to filter [`Custodian`]s by pk
#[diesel::dsl::auto_type(no_type_alias)]
pub fn custodian_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_custodian::id.eq(pk)
}

/// Returns a query fragment to filter [`Fx`]s by pk
#[diesel::dsl::auto_type(no_type_alias)]
pub fn fx_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_fx::id.eq(pk)
}

/// Returns a query fragment to filter [`AccountType`]s that have 'unique_name'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn acounttype_with_unique_name() -> _ {
    crate::schema::finances_accounts_accounttype::unique_name.is_not_null()
}

/// Returns a query fragment to filter [`AccountType`]s by 'unique_name'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn acounttype_by_unique_name(unique_name: &str) -> _ {
    crate::schema::finances_accounts_accounttype::unique_name.eq(unique_name)
}

/// Returns query fragment to filter [`AccountType`]s by 'unique_name'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn accounttype_by_unique_names<'a>(unique_names: &'a [&'a str]) -> _ {
    crate::schema::finances_accounts_accounttype::unique_name.eq_any(unique_names)
}

/// Returns query fragment to filter [`AccountType`]s by 'pk'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn accounttype_by_pks<'a>(pks: &'a [i64]) -> _ {
    crate::schema::finances_accounts_accounttype::id.eq_any(pks)
}

/// Returns query fragment to filter [`MovementType`]s with 'unique_name
#[diesel::dsl::auto_type(no_type_alias)]
pub fn movementtype_with_unique_name() -> _ {
    crate::schema::finances_accounts_movementtype::unique_name.is_not_null()
}

/// Returns query fragment to filter [`MovementType`]s by 'unique_name'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn movementtype_by_unique_name(unique_name: &str) -> _ {
    crate::schema::finances_accounts_movementtype::unique_name.eq(unique_name)
}

/// Returns query fragment to filter [`MovementType`]s by 'pk'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn movementtype_by_pks<'a>(pks: &'a [i64]) -> _ {
    crate::schema::finances_accounts_movementtype::id.eq_any(pks)
}

/// Returns query fragment to get [`MovementType`]s by 'pk'
#[diesel::dsl::auto_type(no_type_alias)]
pub fn movementtype_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_movementtype::id.eq(pk)
}

/// Returns a query fragment to filter [`Movement`]s by account_id
#[diesel::dsl::auto_type]
pub fn movement_filter_account_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_movement::account_id.eq(pk)
}

/// Returns a query fragment to filter [`Movement`]s by account_id
#[diesel::dsl::auto_type]
pub fn movement_filter_transaction_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_movement::transaction_id.eq(pk)
}

/// Returns a query fragment to filter [`Movement`]s by direction
#[diesel::dsl::auto_type]
pub fn movement_filter_by_direction(direction: MovementDirection) -> _ {
    crate::schema::finances_accounts_movement::direction.eq(direction)
}

/// Returns a query fragment to filter [`Snapshot`]s by account_id
#[diesel::dsl::auto_type]
pub fn snapshot_filter_account_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_snapshot::account_id.eq(pk)
}

/// Returns a query fragment to get [`Snapshot`]s by 'pk'
#[diesel::dsl::auto_type]
pub fn snapshot_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_snapshot::id.eq(pk)
}

/// Returns a query fragment to filter [`Account`]s by pk
#[diesel::dsl::auto_type]
pub fn accountholder_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_accountholder::id.eq(pk)
}

/// Returns a query fragment to filter all the [`TransactionGroup`]s that are active as of today (they don't have end date or it is later than today or equal)
#[diesel::dsl::auto_type]
pub fn transaction_group_active() -> _ {
    crate::schema::finances_accounts_transactiongroup::end
        .is_null()
        .or(crate::schema::finances_accounts_transactiongroup::end.ge(diesel::dsl::today))
}

/// Returns a query fragment to filter [`TransactionGroup`]s by pk
#[diesel::dsl::auto_type]
pub fn transactiongroup_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_transactiongroup::id.eq(pk)
}

/// Returns a query fragment to filter [`Transaction`]s by pk
#[diesel::dsl::auto_type]
pub fn transaction_by_pk(pk: i64) -> _ {
    crate::schema::finances_accounts_transaction::id.eq(pk)
}
