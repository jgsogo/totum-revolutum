use crate::google_type;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct SnapshotsRequest(crate::protos::finances_app_models::SnapshotsRequest);

impl SnapshotsRequest {
    pub fn account_pk(&self) -> &i64 {
        &self.0.account_pk
    }

    pub fn start_date(&self) -> Option<&google_type::Date> {
        self.0.start_date.as_ref().map(google_type::Date::new_ref)
    }

    pub fn end_date(&self) -> Option<&google_type::Date> {
        self.0.end_date.as_ref().map(google_type::Date::new_ref)
    }
}
