use super::AppModel;

pub struct MovementType(pub(crate) crate::protos::MovementType);

impl MovementType {
    pub fn new(pk: i64, name: String, breadcrumb: Option<Vec<String>>) -> Self {
        Self(crate::protos::MovementType {
            pk,
            name,
            breadcrumb: breadcrumb.unwrap_or(Vec::default()),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }
}

impl AppModel<crate::protos::MovementType> for MovementType {
    fn inner_type(self) -> crate::protos::MovementType {
        self.0
    }
}
