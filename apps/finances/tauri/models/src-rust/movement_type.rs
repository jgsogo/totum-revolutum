use super::AppModel;

pub struct MovementType(crate::protos::MovementType);

impl MovementType {
    pub fn new(pk: i64, name: String, breadcrumb: Option<Vec<String>>) -> Self {
        Self(crate::protos::MovementType {
            pk,
            name,
            breadcrumb: breadcrumb.unwrap_or(Vec::default()),
        })
    }
}

impl AppModel<crate::protos::MovementType> for MovementType {
    fn inner_type(self) -> crate::protos::MovementType {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::MovementType {
        &self.0
    }
}
