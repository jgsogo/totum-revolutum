use super::AppModel;

pub struct Custodian(crate::protos::Custodian);

impl From<finances_accounts::models::Custodian> for Custodian {
    fn from(value: finances_accounts::models::Custodian) -> Self {
        Self(crate::protos::Custodian {
            pk: value.id,
            name: value.name,
            photo: value.photo,
        })
    }
}

impl AppModel<crate::protos::Custodian> for Custodian {
    fn inner_type(self) -> crate::protos::Custodian {
        self.0
    }
}
