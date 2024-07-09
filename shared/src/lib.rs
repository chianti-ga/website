mod fiche_rp;
mod research_report;

#[cfg(test)]
mod tests {
    use crate::fiche_rp::*;

    #[test]
    fn fiche_rp() {
        let mut fiche: FicheRP = FicheRP {
            discord_id: "".to_string(),
            name: "Roger".to_string(),
            job: Job::Science(ScienceRole::Researcher(ScienceLevel::Senior)),
            description: "Je suis un grand garçon".to_string(),
            lore: "je suis pas réel".to_string(),
            submission_date: 0,
            messages: vec![],
            version: vec![],
        };

        println!("{}", serde_json::to_string(&fiche).unwrap())
    }
}