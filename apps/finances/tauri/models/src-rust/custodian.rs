// pub struct Custodian(pub(crate) crate::protos::finances_app_models::Custodian);

// impl From<finances_accounts::models::Custodian> for Custodian {
//     fn from(value: finances_accounts::models::Custodian) -> Self {
//         Self(crate::protos::finances_app_models::Custodian {
//             pk: value.id,
//             name: value.name,
//             photo: value.photo,
//         })
//     }
// }

use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Custodian(crate::protos::finances_app_models::Custodian);

impl Custodian {
    pub fn new(pk: i64, name: String, photo: Option<String>) -> Self {
        Self(crate::protos::finances_app_models::Custodian { pk, name, photo })
    }

    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn photo(&self) -> Option<&String> {
        self.0.photo.as_ref()
    }
}

impl From<finances_accounts::models::Custodian> for Custodian {
    fn from(value: finances_accounts::models::Custodian) -> Self {
        Self::new(value.id, value.name, value.photo)
    }
}
