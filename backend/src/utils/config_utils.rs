use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Configuration {
    pub port: u16,
    pub webhooks_list: Vec<WebhookEntry>,
    pub mongo_db_uri: String,
    pub oauth2client: Oauth2Client,
}
#[derive(Deserialize)]
#[derive(Clone)]
pub struct WebhookEntry {
    pub webhook: String,
    pub url: String,
    pub name: String,
}
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Oauth2Client {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
}