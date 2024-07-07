mod fiche_rp;
mod research_report;

#[cfg(test)]
mod tests {
    use crate::fiche_rp::*;

    #[test]
    fn fiche_rp() {
        let mut fiche = FicheRP {
            name: "Roger".to_string(),
            job: Job::Science(ScienceRole::Researcher(ScienceLevel::Senior)),
            description: "Je suis un grand garçon".to_string(),
            lore: "je suis pas réel".to_string(),
        };

        println!("{}", fiche.job)
    }
}