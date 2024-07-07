use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Configuration {
    pub port: u16,
    pub webhooks_list: Vec<WebhookEntry>,
    pub mongo_db_uri: String,
}
#[derive(Deserialize)]
#[derive(Clone)]
pub struct WebhookEntry {
    pub webhook: String,
    pub url: String,
    pub name: String,
}