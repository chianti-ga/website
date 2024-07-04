use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Configuration {
    pub port: u16,
    pub text_webhooks: Vec<TextWebhookEntry>,
}
#[derive(Deserialize)]
#[derive(Clone)]
pub struct TextWebhookEntry {
    pub webhook: String,
    pub url: String,
    pub name: String,
}