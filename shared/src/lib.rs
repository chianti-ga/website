pub mod fiche_rp;
pub mod research_report;
pub mod user;
pub mod discord;
pub mod permissions;
pub mod website_meta;

#[cfg(test)]
mod tests {
    use crate::fiche_rp::*;

    #[test]
    fn fiche_rp() {
        let fiche: FicheRP = FicheRP {
            id: "".to_string(),
            name: "Roger".to_string(),
            job: Job::Science(ScienceRole::Researcher(ScienceRank::Senior)),
            description: "Je suis un grand garçon".to_string(),
            lore: "je suis pas réel".to_string(),
            submission_date: 0,
            messages: vec![],
            version: vec![],
            state: FicheState::Waiting,
        };

        println!("{}", serde_json::to_string(&fiche).unwrap())
    }
}