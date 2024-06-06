use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ObjectDoesNotExist(ObjectDoesNotExist),
}

#[derive(Debug, Error)]
#[error("Cannot find '{model}' with the given filters")]
pub struct ObjectDoesNotExist {
    pub model: String,
}
