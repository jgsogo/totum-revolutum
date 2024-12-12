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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_non_numerable_amount() {
        {
            let amount = NewAmount {
                amount: Some(300f32),
                quantity: None,
                unit_value: None,
            };

            let (amount, quantity, unit_value) = amount.into_bigdecimals(false).unwrap();
            assert!(quantity.is_none());
            assert!(unit_value.is_none());
            assert_eq!(amount, 300f32.try_into().unwrap());
        }

        // quantity and unit_value are ignored
        {
            let amount = NewAmount {
                amount: Some(300f32),
                quantity: Some(300f32),
                unit_value: Some(300f32),
            };

            let (amount, quantity, unit_value) = amount.into_bigdecimals(false).unwrap();
            assert!(quantity.is_none());
            assert!(unit_value.is_none());
            assert_eq!(amount, 300f32.try_into().unwrap());
        }

        // if amount is not given, it fails
        {
            let amount = NewAmount {
                amount: None,
                quantity: Some(300f32),
                unit_value: Some(300f32),
            };

            let r = amount.into_bigdecimals(false);
            assert!(r.is_err());
        }
    }

    #[test]
    fn test_numerable_amount() {
        {
            let amount = NewAmount {
                amount: None,
                quantity: Some(3f32),
                unit_value: Some(100f32),
            };

            let (amount, quantity, unit_value) = amount.into_bigdecimals(true).unwrap();
            assert!(quantity.is_some());
            assert!(unit_value.is_some());
            assert_eq!(amount, 300f32.try_into().unwrap());
        }

        // If 'amount' is given, it's ignored
        {
            let amount = NewAmount {
                amount: Some(5000f32),
                quantity: Some(3f32),
                unit_value: Some(100f32),
            };

            let (amount, quantity, unit_value) = amount.into_bigdecimals(true).unwrap();
            assert!(quantity.is_some());
            assert!(unit_value.is_some());
            assert_eq!(amount, 300f32.try_into().unwrap());
        }

        // If 'quantity' is missing, it fails
        {
            let amount = NewAmount {
                amount: Some(5000f32),
                quantity: None,
                unit_value: Some(100f32),
            };

            let r = amount.into_bigdecimals(true);
            assert!(r.is_err());
        }

        // If 'unit_value' is missing, it fails
        {
            let amount = NewAmount {
                amount: Some(5000f32),
                quantity: Some(100f32),
                unit_value: None,
            };

            let r = amount.into_bigdecimals(true);
            assert!(r.is_err());
        }
    }
}
