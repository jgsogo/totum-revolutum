pub struct Custodian(pub(crate) crate::protos::finances_app_models::Custodian);

impl From<finances_accounts::models::Custodian> for Custodian {
    fn from(value: finances_accounts::models::Custodian) -> Self {
        Self(crate::protos::finances_app_models::Custodian {
            pk: value.id,
            name: value.name,
            photo: value.photo,
        })
    }
}
