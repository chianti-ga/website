use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct FicheRP {
    pub id: String,
    pub name: String,
    pub job: Job,
    pub description: String,
    pub lore: String,
    pub submission_date: u64,
    pub messages: Vec<ReviewMessage>,
    pub version: Vec<FicheVersion>,
    pub state: FicheState,
    //TODO:VEC RAPPORTS
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FicheVersion {
    pub name: String,
    pub job: Job,
    pub description: String,
    pub lore: String,
    pub submission_date: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReviewMessage {
    pub discord_id: String,
    pub content: String,
    pub date: u64,
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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum Job {
    Security(SecurityRole),
    Science(ScienceRole),
    ClassD,
    Medic(MedicRole),
    SiteDirector,
    Chaos,
    Other(String),
}
impl Job {
    pub fn get_science_role(&self) -> Option<&ScienceRole> {
        match self {
            Job::Science(role) => Option::from(role),
            _ => None
        }
    }
    pub fn get_security_role(&self) -> Option<&SecurityRole> {
        match self {
            Job::Security(role) => Option::from(role),
            _ => None
        }
    }
    pub fn get_science_level(&self) -> Option<&ScienceRank> {
        match self {
            Job::Science(role) => Option::from(role.get_science_level()),
            _ => None
        }
    }
    pub fn get_security_level(&self) -> Option<&SecurityRank> {
        match self {
            Job::Security(role) => Option::from(role.get_security_level()),
            _ => None
        }
    }

    pub fn get_medic_role(&self) -> Option<&MedicRole> {
        match self {
            Job::Medic(role) => Option::from(role),
            _ => None
        }
    }

    pub fn get_medic_level(&self) -> Option<&MedicRank> {
        match self {
            Job::Medic(role) => Option::from(role.get_medic_level()),
            _ => None
        }
    }

    pub fn get_other_string(&self) -> Option<&String> {
        match self {
            Job::Other(string) => Option::from(string),
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
            Job::Medic(role) => write!(f, "Médecine ({})", role),
            Job::SiteDirector => write!(f, "Directeur du Site"),
            Job::Chaos => write!(f, "Chaos"),
            Job::Other(string) => write!(f, "Autres ({})", string),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum ScienceRole {
    Scientific(ScienceRank),
    Researcher(ScienceRank),
    Doctor(ScienceRank),
    Supervisor(ScienceRank),
}
impl ScienceRole {
    fn get_science_level(&self) -> &ScienceRank {
        return match self {
            ScienceRole::Scientific(level) => level,
            ScienceRole::Researcher(level) => level,
            ScienceRole::Doctor(level) => level,
            ScienceRole::Supervisor(level) => level
        };
    }
}
impl Display for ScienceRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScienceRole::Scientific(level) => write!(f, "Scientifique {}", level),
            ScienceRole::Researcher(level) => write!(f, "Chercheur {}", level),
            ScienceRole::Doctor(level) => write!(f, "Docteur {}", level),
            ScienceRole::Supervisor(level) => write!(f, "Superviseur {}", level),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum SecurityRole {
    SecurityOfficier(SecurityRank),
    Ntf(SecurityRank),
    Gunsmith(SecurityRank),
    TacticalAgent(SecurityRank),
}

impl SecurityRole {
    fn get_security_level(&self) -> &SecurityRank {
        return match self {
            SecurityRole::SecurityOfficier(level) => level,
            SecurityRole::TacticalAgent(level) => level,
            SecurityRole::Gunsmith(level) => level,
            SecurityRole::Ntf(level) => level
        };
    }
}
impl Display for SecurityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityRole::SecurityOfficier(level) => write!(f, "Officier de Sécurité ({})", level),
            SecurityRole::Ntf(level) => write!(f, "Nine-Tailed Fox ({})", level),
            SecurityRole::TacticalAgent(level) => write!(f, "Agent Tactique ({})", level),
            SecurityRole::Gunsmith(level) => write!(f, "Armurier ({})", level)
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, EnumIter, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum MedicRole {
    Director,
    DirectorAdj,
    Manager,
    Psychiatrist(MedicRank),
    Surgeon(MedicRank),
    Doctor(MedicRank),
    Nurse,
}

impl Display for MedicRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MedicRole::Director => write!(f, "Directeur"),
            MedicRole::DirectorAdj => write!(f, "Directeur Adjoint"),
            MedicRole::Manager => write!(f, "Responsable"),
            MedicRole::Doctor(rank) => write!(f, "Médecin {}", rank),
            MedicRole::Psychiatrist(rank) => write!(f, "Psychiatre {}", rank),
            MedicRole::Surgeon(rank) => write!(f, "Chirurgien {}", rank),
            MedicRole::Nurse => write!(f, "Infirmier/Infirmière"),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum MedicRank {
    Beginner,
    Confirmed,
    Senior,
}

impl MedicRole {
    fn get_medic_level(&self) -> Option<&MedicRank> {
        match self {
            MedicRole::Doctor(rank) => Option::from(rank),
            MedicRole::Psychiatrist(rank) => Option::from(rank),
            MedicRole::Surgeon(rank) => Option::from(rank),
            _ => None
        }
    }
}

impl Display for MedicRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MedicRank::Beginner => write!(f, "Junior"),
            MedicRank::Confirmed => write!(f, "Confirmé"),
            MedicRank::Senior => write!(f, "Sénior")
        }
    }
}