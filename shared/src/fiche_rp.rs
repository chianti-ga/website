use std::fmt::{Display, Formatter};

pub struct FicheRP {
    pub name: String,
    pub job: Job,
    pub description: String,
    pub lore: String,
}

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

pub enum SecurityRole {
    Test
}
impl Display for SecurityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityRole::Test => { write!(f, "test fmt") }
        }
    }
}
