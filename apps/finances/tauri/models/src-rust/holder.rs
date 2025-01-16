pub struct Holder(pub(crate) crate::protos::Holder);

impl From<finances_accounts::models::AccountHolder> for Holder {
    fn from(value: finances_accounts::models::AccountHolder) -> Self {
        Self(crate::protos::Holder {
            pk: value.id,
            name: value.name,
            is_company: value.is_company,
            photo: value.photo,
        })
    }
}
