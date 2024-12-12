/// Application state
#[derive(Clone)]
pub struct AppState {
    pub postgres_url: String,
    pub base_url: String,
    media_url: String,
    static_url: String,
    pub base_ccy: String,
}

impl Default for AppState {
    fn default() -> Self {
        let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL envvar is required");
        let base_url = std::env::var("BASE_URL").expect("BASE_URL envvar is required");
        let media_url = std::env::var("MEDIA_URL").expect("MEDIA_URL envvar is required");
        let static_url = std::env::var("STATIC_URL").expect("STATIC_URL envvar is required");
        let base_ccy = "EUR".to_string();

        Self {
            postgres_url,
            base_url,
            media_url,
            static_url,
            base_ccy,
        }
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
        Self {
            postgres_url,
            base_url,
            media_url,
            static_url,
            base_ccy,
        }
    }

    pub fn postgres_url(&self) -> &str {
        &self.postgres_url
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn base_media_url(&self) -> String {
        format!("{}{}", self.base_url, self.media_url)
    }

    pub fn base_static_url(&self) -> String {
        format!("{}{}", self.base_url, self.static_url)
    }
}
