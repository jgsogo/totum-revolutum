pub struct MovementType(pub(crate) crate::protos::MovementType);

impl MovementType {
    pub fn new(pk: i64, name: String, breadcrumb: Option<Vec<String>>) -> Self {
        Self(crate::protos::MovementType {
            pk,
            name,
            breadcrumb: breadcrumb.unwrap_or_default(),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }
}
