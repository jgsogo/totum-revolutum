use crate::google_type::CurrencyCode;
use crate::{Error, Result};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug, Clone)]
pub struct DatabaseConnection(crate::protos::finances_app_models::DatabaseConnection);

impl DatabaseConnection {
    pub fn new(user: &str, password: &str, host: &str, port: u16, dbname: &str) -> Self {
        Self(crate::protos::finances_app_models::DatabaseConnection {
            user: user.to_string(),
            password: password.to_string(),
            host: host.to_string(),
            port: port.into(),
            dbname: dbname.to_string(),
        })
    }

    pub fn user(&self) -> &str {
        &self.0.user
    }

    pub fn password(&self) -> &str {
        &self.0.password
    }

    pub fn host(&self) -> &str {
        &self.0.host
    }

    pub fn port(&self) -> u16 {
        self.0.port.try_into().expect("Port doesn't fit into u16")
    }

    pub fn dbname(&self) -> &str {
        &self.0.dbname
    }

    pub fn postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user(),
            self.password(),
            self.host(),
            self.port(),
            self.dbname()
        )
    }
}

#[repr(transparent)]
#[derive(ProtoWrapper, Debug, Clone)]
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

    pub fn base_url(&self) -> Result<&str> {
        Ok(&self.0.base_url)
    }

    pub fn base_media_url(&self) -> Result<&str> {
        Ok(&self.0.base_media_url)
    }

    pub fn db(&self) -> Result<&DatabaseConnection> {
        self.0
            .db
            .as_ref()
            .map(DatabaseConnection::new_ref)
            .ok_or(Error::MissingRequiredField("db".to_string()))
    }
}
