use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordAuthorizationInformation {
    pub expires: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GuildMember {
    pub roles: Vec<String>,
}
