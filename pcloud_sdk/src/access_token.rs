pub trait OAuth2Token {
    fn hostname(&self) -> String; // TODO: return &str

    fn access_token(&self) -> &str;
}
