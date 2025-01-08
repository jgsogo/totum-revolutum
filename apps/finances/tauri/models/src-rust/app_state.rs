use crate::AppModel;
use prost::Message;

#[derive(Clone)]
pub struct AppState(crate::protos::AppState);

impl AppState {
    pub fn new(
        postgres_url: String,
        base_url: String,
        media_url: String,
        static_url: String,
        base_ccy: String,
    ) -> Self {
        Self(crate::protos::AppState {
            base_ccy: crate::protos::Ccy::from_str_name(&base_ccy)
                .expect("EUR is not a valid CCY")
                .into(),
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
        let ccy: crate::protos::Ccy = self.0.base_ccy.try_into().expect("Invalid i32 for CCY");
        ccy.as_str_name()
    }
}

impl AppModel for AppState {
    fn encode_to_vec(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }
}
