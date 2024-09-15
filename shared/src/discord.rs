use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordAuthorizationInformation {
    pub expires: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct User {
    pub id: String,
    pub global_name: String,
    pub avatar: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GuildMember {
    pub roles: Vec<String>,
}