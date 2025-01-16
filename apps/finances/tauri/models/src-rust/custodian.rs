pub struct Custodian(pub(crate) crate::protos::Custodian);

impl From<finances_accounts::models::Custodian> for Custodian {
    fn from(value: finances_accounts::models::Custodian) -> Self {
        Self(crate::protos::Custodian {
            pk: value.id,
            name: value.name,
            photo: value.photo,
        })
    }
}
