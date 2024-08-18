use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WebsiteMeta {
    pub whitelist: Vec<String>,
}
impl Default for WebsiteMeta {
    fn default() -> Self {
        WebsiteMeta {
            whitelist: vec![],
        }
    }
}