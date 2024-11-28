/// Application state
pub struct AppState {
    pub postgres_url: String,
    pub base_url: String,
    media_root: String,
}

impl Default for AppState {
    fn default() -> Self {
        let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL envvar is required");
        let base_url = std::env::var("BASE_URL").expect("BASE_URL envvar is required");
        let media_root = std::env::var("MEDIA_ROOT").expect("MEDIA_ROOT envvar is required");

        Self {
            postgres_url,
            base_url,
            media_root,
        }
    }
}

impl AppState {
    pub fn postgres_url(&self) -> &str {
        &self.postgres_url
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn base_media_url(&self) -> String {
        format!("{}{}", self.base_url, self.media_root)
    }
}
