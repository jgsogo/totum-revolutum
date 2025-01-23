use super::google_type;

pub struct MoneyAmount(pub(crate) crate::protos::finances_app_models::MoneyAmount);

impl From<MoneyAmount> for crate::protos::finances_app_models::MoneyAmount {
    fn from(val: MoneyAmount) -> Self {
        val.0
    }
}

impl MoneyAmount {
    pub fn new_non_numerable(amount: google_type::Money) -> Self {
        let amount = crate::protos::finances_app_models::money_amount::Amount::NonNumerable(
            crate::protos::finances_app_models::money_amount::NonNumerable {
                amount: Some(amount.into()),
            },
        );

        Self(crate::protos::finances_app_models::MoneyAmount { amount: Some(amount) })
    }

    pub fn new_numerable(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        let amount = crate::protos::finances_app_models::money_amount::Amount::Numerable(
            crate::protos::finances_app_models::money_amount::Numerable {
                unit_value: Some(unit_value.into()),
                quantity: Some(quantity.into()),
            },
        );
        Self(crate::protos::finances_app_models::MoneyAmount { amount: Some(amount) })
    }
}
