use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use crate::user::FrontAccount;

#[derive(Serialize, Deserialize, Clone)]
pub struct FicheRP {
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

impl FicheRP {
    pub fn get_markdown_string(&mut self) -> String {
        format!("**Nom**: {}\n---\n**Job** {}\n---\n**Description** {}\n---\n**Lore du personage** {}", &self.name, &self.job, &self.description, &self.lore)
    }
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
    pub user: FrontAccount,
    pub content: String,
    pub date: u128,
    pub is_private: bool,
    pub set_state: FicheState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
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
}
impl Job {
    pub fn get_science_role(&self) -> Option<&ScienceRole> {
        return match self {
            Job::Security(_) => None,
            Job::Science(role) => Option::from(role),
            Job::ClassD => None,
            Job::Doctor => None,
            Job::SiteDirector => None,
            Job::Chaos => None,
        }
    }
    pub fn get_security_role(&self) -> Option<&SecurityRole> {
        return match self {
            Job::Security(role) => Option::from(role),
            Job::Science(_) => None,
            Job::ClassD => None,
            Job::Doctor => None,
            Job::SiteDirector => None,
            Job::Chaos => None,
        }
    }
    pub fn get_science_level(&self) -> Option<&ScienceLevel> {
        return match self {
            Job::Security(_) => None,
            Job::Science(role) => Option::from(role.get_science_level()),
            Job::ClassD => None,
            Job::Doctor => None,
            Job::SiteDirector => None,
            Job::Chaos => None,
        }
    }
    pub fn get_security_level(&self) -> Option<&SecurityLevel> {
        return match self {
            Job::Security(role) => Option::from(role.get_security_level()),
            Job::Science(_) => None,
            Job::ClassD => None,
            Job::Doctor => None,
            Job::SiteDirector => None,
            Job::Chaos => None,
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

        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum ScienceRole {
    Scientific(ScienceLevel),
    Researcher(ScienceLevel),
    Supervisor(ScienceLevel),
}
impl ScienceRole {
    fn get_science_level(&self) -> &ScienceLevel {
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
pub enum ScienceLevel {
    Beginner,
    Confirmed,
    Senior,
}

impl Display for ScienceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScienceLevel::Beginner => write!(f, "Junior"),
            ScienceLevel::Confirmed => write!(f, "Confirmé"),
            ScienceLevel::Senior => write!(f, "Sénior"),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum SecurityRole {
    SecurityOfficier(SecurityLevel),
    TacticalAgent(SecurityLevel),
}

impl SecurityRole {
    fn get_security_level(&self) -> &SecurityLevel {
        return match self {
            SecurityRole::SecurityOfficier(level) => level,
            SecurityRole::TacticalAgent(level) => level,
        }
    }
}
impl Display for SecurityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityRole::SecurityOfficier(level) => write!(f, "Officier de Sécurité ({})", level),
            SecurityRole::TacticalAgent(level) => write!(f, "Agent Tactique ({})", level)
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, EnumIter)]
pub enum SecurityLevel {
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
impl Display for SecurityLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityLevel::Rct => write!(f, "Recrue"),
            SecurityLevel::Sdt => write!(f, "Soldat"),
            SecurityLevel::sdt => write!(f, "Première Classe"),
            SecurityLevel::cpl => write!(f, "Caporal"),
            SecurityLevel::cplC => write!(f, "Caporal-Chef"),
            SecurityLevel::CplC1c => write!(f, "Caporal-Chef Première Classe"),
            SecurityLevel::Sgt => write!(f, "Sergent"),
            SecurityLevel::SgtC => write!(f, "Sergent-Chef"),
            SecurityLevel::Adj => write!(f, "Adjudant"),
            SecurityLevel::AdjC => write!(f, "Adjudant-Chef"),
            SecurityLevel::Maj => write!(f, "Major"),
            SecurityLevel::Asp => write!(f, "Aspirant"),
            SecurityLevel::Slt => write!(f, "Sous-Lieutenant"),
            SecurityLevel::Lt => write!(f, "Lieutenant"),
            SecurityLevel::Cpt => write!(f, "Capitaine"),
            SecurityLevel::Cmd => write!(f, "Commandant"),
            SecurityLevel::LtCol => write!(f, "Lieutenant-Colonel"),
            SecurityLevel::Col => write!(f, "Colonel"),
            SecurityLevel::Gen => write!(f, "Général"),
        }
    }
}
