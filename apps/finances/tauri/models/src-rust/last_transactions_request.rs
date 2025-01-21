use super::MovementDirection;

use prost::Message;

pub struct LastTransactionsRequest(crate::protos::finances_app_models::LastTransactionsRequest);

impl TryFrom<Vec<u8>> for LastTransactionsRequest {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(
            crate::protos::finances_app_models::LastTransactionsRequest::decode(&*v)?,
        ))
    }
}

impl LastTransactionsRequest {
    pub fn account_pk(&self) -> i64 {
        self.0.account_pk
    }

    pub fn account_movement_direction(&self) -> MovementDirection {
        self.0
            .account_movement_direction
            .try_into()
            .expect("Invalid MovementDirection value")
    }
}
