use thiserror::Error;

#[derive(Error, Debug)]
pub enum SDKErrors {
    #[error("can't get lock for project path '{0:?}'")]
    ProjectLocked(String),

    #[error("not implemented")]
    NotImplemented,
}
