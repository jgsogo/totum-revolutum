use crate::movement::MovementDirection;

use crate::Result;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct LastTransactionsRequest(crate::protos::finances_app_models::LastTransactionsRequest);

impl LastTransactionsRequest {
    pub fn account_pk(&self) -> &i64 {
        &self.0.account_pk
    }

    pub fn account_movement_direction(&self) -> Result<MovementDirection> {
        Ok(self.0.account_movement_direction.try_into()?)
    }
}
