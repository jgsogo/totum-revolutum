use super::{google_type, AppModel};

pub struct Fx(crate::protos::Fx);

impl Fx {
    pub fn new(
        foreign_code: String,
        local_code: String,
        date_value: google_type::Date,
        fx: google_type::Decimal,
    ) -> Self {
        Self(crate::protos::Fx {
            date_value: Some(date_value.into()),
            foreign_code,
            local_code,
            fx: Some(fx.into()),
        })
    }
}

impl AppModel<crate::protos::Fx> for Fx {
    fn inner_type(self) -> crate::protos::Fx {
        self.0
    }
}
