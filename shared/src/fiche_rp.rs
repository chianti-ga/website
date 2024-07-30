use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::user::UserAccount;

#[derive(Serialize, Deserialize)]
pub struct FicheRP {
    pub name: String,
    pub job: Job,
    pub description: String,
    pub lore: String,
    pub submission_date: u128,
    pub messages: Vec<ReviewMessage>,
    pub version: Vec<FicheVersions>
    pub state: FicheState,
    //TODO:VEC RAPPORTS
}

impl FicheRP {
    pub fn get_markdown_string(&mut self) -> String {
        format!("**Nom**: {}\n---\n**Job** {}\n---\n**Description** {}\n---\n**Lore du personage** {}", &self.name, &self.job, &self.description, &self.lore)
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ReviewMessage {
    pub user: UserAccount,
    pub content: String,
    pub date: u128,
    pub is_private: bool,
    pub set_state: FicheState,
}

#[derive(Serialize, Deserialize)]
pub enum FicheState {
    Waiting,
    RequestModification,
    StaffValidated,
    Accepted,
    Refused,
    Comment

}

/**     JOB INFO STARTS HERE    **/
#[derive(Serialize, Deserialize)]
pub enum Job {
    Security(SecurityRole),
    Science(ScienceRole),
    ClassD,
    Doctor,
    SiteDirector,
    Chaos,
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
#[derive(Serialize, Deserialize)]

pub enum ScienceRole {
    Assistant(ScienceLevel),
    Researcher(ScienceLevel),
    Supervisor(ScienceLevel),
}
impl Display for ScienceRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScienceRole::Assistant(level) => write!(f, "Assitant {}", level),
            ScienceRole::Researcher(level) => write!(f, "Chercheur {}", level),
            ScienceRole::Supervisor(level) => write!(f, "Superviseur {}", level),
        }
    }
}
#[derive(Serialize, Deserialize)]

pub enum ScienceLevel {
    Beginner,
    Confirmed,
    Senior,
}
impl Display for ScienceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScienceLevel::Beginner => write!(f, "Débutant"),
            ScienceLevel::Confirmed => write!(f, "Confirmé"),
            ScienceLevel::Senior => write!(f, "Sénior")
        }
    }
}
#[derive(Serialize, Deserialize)]
pub enum SecurityRole {
    SecurityOfficier(SecurityLevel),
    TacticalAgent(SecurityLevel)
}
impl Display for SecurityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityRole::SecurityOfficier(level) => write!(f, "Officier de Sécurité ({})", level),
            SecurityRole::TacticalAgent(level) => write!(f, "Agent Tactique ({})", level)
        }
    }
}
#[derive(Serialize, Deserialize)]
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
            SecurityLevel::Gen => write!(f, "Général")
        }
    }
}
