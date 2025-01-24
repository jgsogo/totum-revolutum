use crate::{google_type::CurrencyCode, Error, Result};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct DatabaseConnection(crate::protos::finances_app_models::DatabaseConnection);

impl DatabaseConnection {
    pub fn new(postgres_url: String) -> Self {
        Self(crate::protos::finances_app_models::DatabaseConnection { postgres_url })
    }

    pub fn postgres_url(&self) -> &str {
        &self.0.postgres_url
    }
}

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct AppState(crate::protos::finances_app_models::AppState);

impl AppState {
    pub fn new(
        base_ccy: CurrencyCode,
        base_media_url: String,
        base_static_url: String,
        base_url: String,
        db: DatabaseConnection,
    ) -> Self {
        Self(crate::protos::finances_app_models::AppState {
            base_ccy: base_ccy.to_string(),
            base_media_url,
            base_static_url,
            base_url,
            db: Some(db.into()),
        })
    }

    pub fn base_ccy(&self) -> Result<CurrencyCode> {
        Ok(CurrencyCode::new(&self.0.base_ccy)?)
    }

    // pub fn new_from_env() -> Self {
    //     let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL envvar is required");
    //     let base_url = std::env::var("BASE_URL").expect("BASE_URL envvar is required");
    //     let media_url = std::env::var("MEDIA_URL").expect("MEDIA_URL envvar is required");
    //     let static_url = std::env::var("STATIC_URL").expect("STATIC_URL envvar is required");
    //     Self::new(postgres_url, base_url, media_url, static_url, "EUR".to_string())
    // }
}
