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
    pub snapshot: Snapshot,
    pub snapshot_numerable: _SnapshotNumerable,
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

impl NewSnapshotNumerable<'_> {
    pub fn insert_into_db(&self, conn: &mut PgConnection) -> std::result::Result<i64, diesel::result::Error> {
        conn.transaction(|conn| {
            let inner_snapshot_pk = self.new_snapshot.insert_into_db(conn)?;

            let new_movement_numerable = _NewSnapshotNumerable {
                snapshot_ptr_id: &inner_snapshot_pk,
                quantity: self.quantity,
                unit_value: self.unit_value,
            };
            diesel::insert_into(crate::schema::finances_investments_snapshotnumerable::table)
                .values(&new_movement_numerable)
                .returning(crate::schema::finances_investments_snapshotnumerable::snapshot_ptr_id)
                .get_result::<i64>(conn)
        })
    }
}
