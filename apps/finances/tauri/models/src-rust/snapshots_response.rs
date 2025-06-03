use crate::google_type;
use crate::Snapshot;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct SnapshotsResponse(crate::protos::finances_app_models::SnapshotsResponse);

impl SnapshotsResponse {
    pub fn new(
        account_pk: i64,
        snapshots: Vec<Snapshot>,
        start_date: Option<google_type::Date>,
        end_date: Option<google_type::Date>,
    ) -> Self {
        Self(crate::protos::finances_app_models::SnapshotsResponse {
            account_pk,
            snapshots: snapshots.into_iter().map(|v| v.into()).collect(),
            start_date: start_date.map(|v| v.into()),
            end_date: end_date.map(|v| v.into()),
        })
    }
}
