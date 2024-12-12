use serde::{Deserialize, Serialize};

use bigdecimal::BigDecimal;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewAmount {
    pub amount: Option<f32>,
    pub quantity: Option<f32>,
    pub unit_value: Option<f32>,
}

impl NewAmount {
    pub fn into_bigdecimals(
        &self,
        is_numerable: bool,
    ) -> Result<(BigDecimal, Option<BigDecimal>, Option<BigDecimal>), String> {
        if !is_numerable {
            let amount = self
                .amount
                .ok_or("No amount for movement".to_string())?
                .try_into()
                .map_err(|e| {
                    format!(
                        "Cannot convert amount f32 ({}) to BigDecimal: {e}",
                        self.amount.unwrap()
                    )
                })?;
            Ok((amount, None, None))
        } else {
            let quantity = self
                .quantity
                .ok_or("No quantity for movement".to_string())?
                .try_into()
                .map_err(|e| {
                    format!(
                        "Cannot convert quantity f32 ({}) to BigDecimal: {e}",
                        self.quantity.unwrap()
                    )
                })?;

            let unit_value = self
                .unit_value
                .ok_or("No unit_value for movement".to_string())?
                .try_into()
                .map_err(|e| {
                    format!(
                        "Cannot convert unit_value f32 ({}) to BigDecimal: {e}",
                        self.unit_value.unwrap()
                    )
                })?;

            Ok((&quantity * &unit_value, Some(quantity), Some(unit_value)))
        }
    }
}
