use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Holder(crate::protos::finances_app_models::Holder);

impl Holder {
    pub fn new(pk: i64, name: String, is_company: bool, photo: String) -> Self {
        Self(crate::protos::finances_app_models::Holder {
            pk,
            name,
            is_company,
            photo: Some(photo),
        })
    }

    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn is_company(&self) -> bool {
        self.0.is_company
    }

    pub fn photo(&self) -> Option<&str> {
        self.0.photo.as_ref().map(|x| x.as_str())
    }
}
