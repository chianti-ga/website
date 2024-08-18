use std::fmt;
use serde::{Deserialize, Serialize};
use crate::fiche_rp::{FicheState, FicheVersion, Job, ReviewMessage};

#[derive(Clone, PartialEq, Eq)]
pub enum DiscordRole {
    Admin,
    Moderator,
    LeadScenarist,
    Scenarist,
    Unknown
}

impl DiscordRole {
    // Function to get the role ID as a string
    pub fn role_id(&self) -> &str {
        match self {
            DiscordRole::Admin => "1031296249254658138", // TA role
            DiscordRole::Moderator => "1259573584767090699", //Gm mod
            DiscordRole::LeadScenarist => "1143632282926727328", // Resp scenarist
            DiscordRole::Scenarist => "1143509784591605841", // scenarist
            _ => "none"
        }
    }

    pub fn from_role_id(role_id: &str) -> Option<Self> {
        match role_id {
            "1031296249254658138" => Some(DiscordRole::Admin),
            "1259573584767090699" => Some(DiscordRole::Moderator),
            "1143632282926727328" => Some(DiscordRole::LeadScenarist),
            "1143509784591605841" => Some(DiscordRole::Scenarist),
            _ => None,
        }
    }

    pub fn role_summary(&self) -> &str {
        match self {
            DiscordRole::Admin => "Cette personne est un administrateur.",
            DiscordRole::Moderator => "Cette personne est chargée d’appliquer la modération.",
            DiscordRole::LeadScenarist => "Cette personne est un chef scénariste.\nIl décide de l'acceptation finale de votre fiche.",
            DiscordRole::Scenarist => "Cette personne est un scénariste.\n Il donne son avis sur votre fiche.\nIl peut vous demander des modifications ou la refuser.",
            _ => "unknown role"
        }
    }

    pub fn from_role_ids(role_ids: &Vec<String>) -> Option<Vec<Self>> {
        Some(role_ids.into_iter()
                     .filter_map(|id| DiscordRole::from_role_id(&id))
                     .collect())
    }

    // Function to compare the role with a string containing a role ID
    pub fn matches_role_id(&self, role_id: &str) -> bool {
        self.role_id() == role_id
    }
}

impl fmt::Display for DiscordRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.role_summary())
    }
}