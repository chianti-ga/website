#[cfg(target_arch = "x86_64")]
use oauth2::basic::BasicTokenResponse;
use serde::{Deserialize, Serialize};

use crate::discord::User;
use crate::fiche_rp::FicheRP;

#[cfg(target_arch = "x86_64")]
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub discord_user: User,
    pub discord_roles: Vec<String>,
    pub auth_id: String,
    pub token: BasicTokenResponse,
    pub last_renewal: u64,
    pub fiches: Vec<FicheRP>,
    pub creation_date: u64,
    pub banned: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FrontAccount {
    pub discord_user: User,
    pub discord_roles: Vec<String>,
    pub fiches: Vec<FicheRP>,
    pub creation_date: u64,
    pub banned: bool,
}