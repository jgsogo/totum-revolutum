use super::google_type;

pub struct MoneyAmount(pub(crate) crate::protos::MoneyAmount);

impl MoneyAmount {
    pub fn new_non_numerable(amount: google_type::Money) -> Self {
        let amount = crate::protos::money_amount::Amount::NonNumerable(crate::protos::money_amount::NonNumerable {
            amount: Some(amount.into()),
        });

        Self(crate::protos::MoneyAmount { amount: Some(amount) })
    }

    pub fn new_numerable(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        let amount = crate::protos::money_amount::Amount::Numerable(crate::protos::money_amount::Numerable {
            unit_value: Some(unit_value.into()),
            quantity: Some(quantity.into()),
        });
        Self(crate::protos::MoneyAmount { amount: Some(amount) })
    }
}

impl Into<crate::protos::MoneyAmount> for MoneyAmount {
    fn into(self) -> crate::protos::MoneyAmount {
        self.0
    }
}
