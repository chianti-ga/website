use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use crate::user::FrontAccount;

#[derive(Serialize, Deserialize, Clone)]
pub struct FicheRP {
    pub id: String,
    pub name: String,
    pub job: Job,
    pub description: String,
    pub lore: String,
    pub submission_date: u64,
    pub messages: Vec<ReviewMessage>,
    pub version: Vec<FicheVersions>,
    pub state: FicheState,
    //TODO:VEC RAPPORTS
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FicheVersions {
    pub name: String,
    pub job: Job,
    pub description: String,
    pub lore: String,
    pub submission_date: u128,
}

impl FicheVersions {
    pub fn get_markdown_string(&mut self) -> String {
        format!("**Nom**: {}\n---\n**Job** {}\n---\n**Description** {}\n---\n**Lore du personage** {}", &self.name, &self.job, &self.description, &self.lore)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReviewMessage {
    pub account: FrontAccount,
    pub content: String,
    pub date: u128,
    pub is_private: bool,
    pub is_comment: bool,
    pub set_state: FicheState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, EnumIter)]
pub enum FicheState {
    Waiting,
    RequestModification,
    StaffValidated,
    Accepted,
    Refused,
    Comment,
}
impl FicheState {
    pub fn get_text(&self) -> &str {
        match self {
            FicheState::Waiting => "EN ATTENTE",
            FicheState::RequestModification => "DEMANDE DE MODIFICATIONS",
            FicheState::StaffValidated => "CONFORME",
            FicheState::Accepted => "ACCEPTÉE",
            FicheState::Refused => "REFUSÉE",
            FicheState::Comment => "COMMENTAIRE"
        }
    }
}

/**     JOB INFO STARTS HERE    **/
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Job {
    Security(SecurityRole),
    Science(ScienceRole),
    ClassD,
    Doctor,
    SiteDirector,
    Chaos,
    Other(String)
}
impl Job {
    pub fn get_science_role(&self) -> Option<&ScienceRole> {
        return match self {
            Job::Science(role) => Option::from(role),
            _ => None
        }
    }
    pub fn get_security_role(&self) -> Option<&SecurityRole> {
        return match self {
            Job::Security(role) => Option::from(role),
            _ => None
        }
    }
    pub fn get_science_level(&self) -> Option<&ScienceRank> {
        return match self {
            Job::Science(role) => Option::from(role.get_science_level()),
            _ => None
        }
    }
    pub fn get_security_level(&self) -> Option<&SecurityRank> {
        return match self {
            Job::Security(role) => Option::from(role.get_security_level()),
            _ => None
        }
    }
}
impl Display for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Job::Security(role) => write!(f, "Sécurité ({})", role),
            Job::Science(role) => write!(f, "Science ({})", role),
            Job::ClassD => write!(f, "Classe-D"),
            Job::Doctor => write!(f, "Médecin"),
            Job::SiteDirector => write!(f, "Directeur du Site"),
            Job::Chaos => write!(f, "Chaos"),
            Job::Other(string) => write!(f, "Autres ({})", string),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum ScienceRole {
    Scientific(ScienceRank),
    Researcher(ScienceRank),
    Supervisor(ScienceRank),
}
impl ScienceRole {
    fn get_science_level(&self) -> &ScienceRank {
        return match self {
            ScienceRole::Scientific(level) => level,
            ScienceRole::Researcher(level) => level,
            ScienceRole::Supervisor(level) => level
        }
    }
}
impl Display for ScienceRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScienceRole::Scientific(level) => write!(f, "Scientifique {}", level),
            ScienceRole::Researcher(level) => write!(f, "Chercheur {}", level),
            ScienceRole::Supervisor(level) => write!(f, "Superviseur {}", level),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum ScienceRank {
    Beginner,
    NoLevel,
    Senior,
}

impl Display for ScienceRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScienceRank::Beginner => write!(f, "Junior"),
            ScienceRank::NoLevel => write!(f, "[Aucun Grade]"),
            ScienceRank::Senior => write!(f, "Sénior"),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum SecurityRole {
    SecurityOfficier(SecurityRank),
    Gunsmith(SecurityRank),
    TacticalAgent(SecurityRank),
}

impl SecurityRole {
    fn get_security_level(&self) -> &SecurityRank {
        return match self {
            SecurityRole::SecurityOfficier(level) => level,
            SecurityRole::TacticalAgent(level) => level,
            SecurityRole::Gunsmith(level) => level,

        }
    }
}
impl Display for SecurityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityRole::SecurityOfficier(level) => write!(f, "Officier de Sécurité ({})", level),
            SecurityRole::TacticalAgent(level) => write!(f, "Agent Tactique ({})", level),
            SecurityRole::Gunsmith(level) => write!(f, "Armurier ({})", level)
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, EnumIter)]
pub enum SecurityRank {
    Rct,
    Sdt,
    sdt,
    cpl,
    cplC,
    CplC1c,
    Sgt,
    SgtC,
    Adj,
    AdjC,
    Maj,
    Asp,
    Slt,
    Lt,
    Cpt,
    Cmd,
    LtCol,
    Col,
    Gen,
}
impl Display for SecurityRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityRank::Rct => write!(f, "Recrue"),
            SecurityRank::Sdt => write!(f, "Soldat"),
            SecurityRank::sdt => write!(f, "Première Classe"),
            SecurityRank::cpl => write!(f, "Caporal"),
            SecurityRank::cplC => write!(f, "Caporal-Chef"),
            SecurityRank::CplC1c => write!(f, "Caporal-Chef Première Classe"),
            SecurityRank::Sgt => write!(f, "Sergent"),
            SecurityRank::SgtC => write!(f, "Sergent-Chef"),
            SecurityRank::Adj => write!(f, "Adjudant"),
            SecurityRank::AdjC => write!(f, "Adjudant-Chef"),
            SecurityRank::Maj => write!(f, "Major"),
            SecurityRank::Asp => write!(f, "Aspirant"),
            SecurityRank::Slt => write!(f, "Sous-Lieutenant"),
            SecurityRank::Lt => write!(f, "Lieutenant"),
            SecurityRank::Cpt => write!(f, "Capitaine"),
            SecurityRank::Cmd => write!(f, "Commandant"),
            SecurityRank::LtCol => write!(f, "Lieutenant-Colonel"),
            SecurityRank::Col => write!(f, "Colonel"),
            SecurityRank::Gen => write!(f, "Général"),
        }
    }
}
