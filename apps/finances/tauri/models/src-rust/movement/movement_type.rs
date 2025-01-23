use crate::traits::ProtoWrapper;

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

impl ProtoWrapper<crate::protos::finances_app_models::MovementType> for MovementType {
    fn as_proto(&self) -> &crate::protos::finances_app_models::MovementType {
        &self.0
    }
}

impl From<crate::protos::finances_app_models::MovementType> for MovementType {
    fn from(value: crate::protos::finances_app_models::MovementType) -> Self {
        Self(value)
    }
}

impl From<MovementType> for crate::protos::finances_app_models::MovementType {
    fn from(val: MovementType) -> Self {
        val.0
    }
}
