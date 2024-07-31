use oauth2::basic::BasicTokenResponse;
use serde::{Deserialize, Serialize};

use crate::discord::User;
use crate::fiche_rp::FicheRP;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub discord_user: User,
    pub token: BasicTokenResponse,
    pub last_renewal: u64,
    pub fiches: Vec<FicheRP>,
    pub creation_date: u64,
    pub banned: bool,
}