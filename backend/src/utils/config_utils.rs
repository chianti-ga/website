use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Configuration {
    pub port: u16,
    pub domain: String,
    pub scena_webhook: String,
    pub mongo_db_uri: String,
    pub oauth2client: Oauth2Client,
}
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Oauth2Client {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub redirect_url: String,
    pub redirect_url_egui: String,
    pub token_url: String,
}