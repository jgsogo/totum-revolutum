use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct MovementType(crate::protos::finances_app_models::MovementType);

impl MovementType {
    pub fn new(pk: i64, name: String, breadcrumb: Option<Vec<String>>) -> Self {
        Self(crate::protos::finances_app_models::MovementType {
            pk,
            name,
            breadcrumb: breadcrumb.unwrap_or_default(),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn breadcrumb(&self) -> Option<&Vec<String>> {
        if self.0.breadcrumb.is_empty() {
            None
        } else {
            Some(&self.0.breadcrumb)
        }
    }
}
