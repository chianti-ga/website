use serde::{Deserialize, Serialize};

use crate::fiche_rp::FicheRP;

#[derive(Serialize, Deserialize)]
pub struct UserAccount {
    pub name: String,
    pub token: String,
    pub fiches: Vec<FicheRP>,
    pub creation_date: u128,
}