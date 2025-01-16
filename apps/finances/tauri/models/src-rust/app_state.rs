use prost::Message;

use super::{AppModel, OutgoingModel};

#[derive(Clone)]
pub struct AppState(crate::protos::AppState);

impl TryFrom<Vec<u8>> for AppState {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::AppState::decode(&*v)?))
    }
}

impl AppState {
    pub fn new(
        postgres_url: String,
        base_url: String,
        media_url: String,
        static_url: String,
        base_ccy: String,
    ) -> Self {
        Self(crate::protos::AppState {
            base_ccy,
            base_media_url: format!("{}{}", base_url, media_url),
            base_static_url: format!("{}{}", base_url, static_url),
            base_url,
            db: Some(crate::protos::DatabaseConnection { postgres_url }),
        })
    }

    pub fn new_from_env() -> Self {
        let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL envvar is required");
        let base_url = std::env::var("BASE_URL").expect("BASE_URL envvar is required");
        let media_url = std::env::var("MEDIA_URL").expect("MEDIA_URL envvar is required");
        let static_url = std::env::var("STATIC_URL").expect("STATIC_URL envvar is required");
        Self::new(postgres_url, base_url, media_url, static_url, "EUR".to_string())
    }

    pub fn postgres_url(&self) -> &str {
        &self.0.db.as_ref().unwrap().postgres_url
    }

    pub fn base_ccy(&self) -> &str {
        &self.0.base_ccy
    }
}

impl AppModel<crate::protos::AppState> for AppState {
    fn inner_type(self) -> crate::protos::AppState {
        self.0
    }
}

impl OutgoingModel for AppState {
    fn as_message(&self) -> &impl Message {
        &self.0
    }
}
