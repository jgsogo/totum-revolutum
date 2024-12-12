use diesel::prelude::*;

use finances_accounts::types::NumericType;

use finances_accounts::models::{NewSnapshot, Snapshot};

use diesel::deserialize::Result;
use diesel::row::NamedRow;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, QueryableByName)]
#[diesel(table_name = crate::schema::finances_investments_snapshotnumerable)]
#[diesel(primary_key(snapshot_ptr_id))]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Snapshot, foreign_key = snapshot_ptr_id))]
pub struct _SnapshotNumerable {
    pub snapshot_ptr_id: i64,
    pub quantity: NumericType,
    pub unit_value: NumericType,
}

pub struct SnapshotNumerable {
    #[allow(dead_code)]
    snapshot: Snapshot,
    #[allow(dead_code)]
    snapshot_numerable: _SnapshotNumerable,
}

impl QueryableByName<finances_accounts::types::BackendType> for SnapshotNumerable
where
    Self: Sized,
{
    fn build<'a>(row: &impl NamedRow<'a, finances_accounts::types::BackendType>) -> Result<Self> {
        let snapshot = <Snapshot as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        let snapshot_numerable =
            <_SnapshotNumerable as diesel::QueryableByName<finances_accounts::types::BackendType>>::build(row)?;
        Ok(SnapshotNumerable {
            snapshot,
            snapshot_numerable,
        })
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_investments_snapshotnumerable)]
pub struct _NewSnapshotNumerable<'a> {
    pub snapshot_ptr_id: &'a i64,
    pub quantity: &'a NumericType,
    pub unit_value: &'a NumericType,
}

pub struct NewSnapshotNumerable<'a> {
    pub new_snapshot: &'a NewSnapshot<'a>,
    pub quantity: &'a NumericType,
    pub unit_value: &'a NumericType,
}
