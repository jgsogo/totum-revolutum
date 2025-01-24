mod movement_amount;
mod movement_direction;
mod movement_type;

use crate::{Error, Result};
pub use movement_amount::MovementAmount;
pub use movement_direction::MovementDirection;
pub use movement_type::MovementType;

use proto_wrapper::ProtoWrapper;

use crate::{google_type, FxQuote};

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Movement(crate::protos::finances_app_models::Movement);

impl Movement {
    pub fn new(
        pk: i64,
        date_value: google_type::Date,
        transaction_pk: i64,
        r#type: MovementType,
        direction: MovementDirection,
        amount: MovementAmount,
        fx: Option<FxQuote>,
    ) -> Self {
        Self(crate::protos::finances_app_models::Movement {
            pk,
            date_value: Some(date_value.into()),
            transaction_pk,
            r#type: Some(r#type.into()),
            direction: direction.to_i32(),
            amount: Some(amount.into()),
            fx: fx.map(|v| v.into()),
        })
    }

    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn date_value(&self) -> Result<&google_type::Date> {
        self.0
            .date_value
            .as_ref()
            .map(google_type::Date::new_ref)
            .ok_or(Error::MissingRequiredField("date_value".to_string()))
    }

    pub fn transaction_pk(&self) -> &i64 {
        &self.0.transaction_pk
    }

    pub fn r#type(&self) -> Result<&MovementType> {
        self.0
            .r#type
            .as_ref()
            .map(MovementType::new_ref)
            .ok_or(Error::MissingRequiredField("type".to_string()))
    }

    pub fn direction(&self) -> Result<MovementDirection> {
        Ok(self.0.direction.try_into()?)
    }

    pub fn movement_amount(&self) -> Result<&MovementAmount> {
        self.0
            .amount
            .as_ref()
            .map(MovementAmount::new_ref)
            .ok_or(Error::MissingRequiredField("amount".to_string()))
    }

    pub fn fx(&self) -> Option<&FxQuote> {
        self.0.fx.as_ref().map(FxQuote::new_ref)
    }

    /// Returns the Money amount in the base currency (use [`Self::movement_amount`] to get the raw information)
    pub fn amount(&self) -> Result<google_type::Money> {
        let amount_raw = self
            .0
            .amount
            .as_ref()
            .map(MovementAmount::new_ref)
            .ok_or(Error::MissingRequiredField("amount".to_string()))?
            .amount()?;

        match self.fx() {
            Some(fx) => &amount_raw * fx,
            None => Ok(amount_raw),
        }
    }
}
