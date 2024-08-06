use std::fmt;

enum DiscordRole {
    Admin,
    Moderator,
    LeadScenarist,
    Scenarist,
    Skitou,
}

impl DiscordRole {
    // Function to get the role ID as a string
    fn role_id(&self) -> &str {
        match self {
            DiscordRole::Admin => "1031296249254658138", // TA role
            DiscordRole::Moderator => "1259573584767090699", //Gm mod
            DiscordRole::LeadScenarist => "1143632282926727328", // Resp scenarist
            DiscordRole::Scenarist => "1143509784591605841", // scenarist
            DiscordRole::Skitou => "374283393799553036", // :D
        }
    }

    fn role_summary(&self) -> &str {
        match self {
            DiscordRole::Admin => "Cette personne est un administrateur.",
            DiscordRole::Moderator => "Cette personne est chargée d’appliquer la modération.",
            DiscordRole::LeadScenarist => "Cette personne est un chef scénariste.\nIl décide de l'acceptation finale de votre fiche.",
            DiscordRole::Scenarist => "Cette personne est un scénariste.\n Il donne son avis sur votre fiche.\nIl peut vous demander des modifications",
            DiscordRole::Skitou => "Developpeur",
            _ => "unknown role"
        }
    }

    // Function to compare the role with a string containing a role ID
    fn matches_role_id(&self, role_id: &str) -> bool {
        return if DiscordRole::Skitou.role_id() == role_id {
            true
        } else {
            self.role_id() == role_id
        };
    }
}

impl fmt::Display for DiscordRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.role_summary())
    }
}