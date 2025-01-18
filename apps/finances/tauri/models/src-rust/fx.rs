use super::google_type;

pub struct Fx(pub(crate) crate::protos::finances_app_models::Fx);

impl Fx {
    pub fn new(
        foreign_code: String,
        local_code: String,
        date_value: google_type::Date,
        fx: google_type::Decimal,
    ) -> Self {
        Self(crate::protos::finances_app_models::Fx {
            date_value: Some(date_value.0),
            foreign_code,
            local_code,
            fx: Some(fx.0),
        })
    }
}
