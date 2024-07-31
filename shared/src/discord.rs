use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordAuthorizationInformation {
    pub expires: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: String,
}

