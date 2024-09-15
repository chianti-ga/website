use crate::CONFIG;
use serenity::all::{CreateEmbed, CreateEmbedFooter, ExecuteWebhook, Http, Webhook};
use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, ReviewMessage};

pub async fn send_scena_fiche_notif(fiche: FicheRP, user: User) {
    actix_rt::spawn(async move {
        let http: Http = Http::new("");

        let webhook: Webhook = Webhook::from_url(&http, &CONFIG.scena_webhook).await?;

        let mut embed: CreateEmbed = CreateEmbed::new()
            .title(format!("Nouvelle FicheRP de {} ", user.global_name))
            .description(format!("Name : **{}** \nJob : **{}**", user.global_name, fiche.job))
            .url("https://intranet.projectvisualis.fr/")
            .colour(0x1F8B4C)
            .footer(CreateEmbedFooter::new("Gestionaire de FicheRP"));

        match fiche.state {
            FicheState::Waiting => embed = embed.image("https://intranet.projectvisualis.fr/app_img/waiting.svg"),
            FicheState::RequestModification => embed = embed.image("https://intranet.projectvisualis.fr/app_img/odif.svg").title(format!("Modificaion FicheRP pour {} ", user.global_name)),
            FicheState::StaffValidated => embed = embed.image("https://intranet.projectvisualis.fr/app_img/conform.svg"),
            FicheState::Accepted => embed = embed.image("https://intranet.projectvisualis.fr/app_img/accepted.svg"),
            FicheState::Refused => embed = embed.image("https://intranet.projectvisualis.fr/app_img/refused.svg"),
            FicheState::Comment => embed = embed.image("https://intranet.projectvisualis.fr/app_img/comment.svg"),
        }

        let builder = ExecuteWebhook::new()
            .embed(embed)
            .username("FicheRP");

        webhook.execute(&http, false, builder).await
    });
}

pub async fn send_scena_comment_notif(fiche: FicheRP, review_message: ReviewMessage, user: User) {
    actix_rt::spawn(async move {
        let http: Http = Http::new("");

        let webhook: Webhook = Webhook::from_url(&http, &CONFIG.scena_webhook).await?;

        let mut embed: CreateEmbed = CreateEmbed::new()
            .title(format!("Nouveau commentaire de {} ", user.global_name))
            .description(format!("**Sur la fiche de :**\nName : **{}** \tJob : **{}**\n {}", user.global_name, fiche.job, review_message.content))
            .url("https://intranet.projectvisualis.fr/")
            .colour(0x1F8B4C)
            .image("https://intranet.projectvisualis.fr/app_img/comment.svg")
            .footer(CreateEmbedFooter::new("Gestionaire de FicheRP"));

        match review_message.set_state {
            FicheState::Waiting => embed = embed.image("https://intranet.projectvisualis.fr/app_img/waiting.svg"),
            FicheState::RequestModification => embed = embed.image("https://intranet.projectvisualis.fr/app_img/odif.svg"),
            FicheState::StaffValidated => embed = embed.image("https://intranet.projectvisualis.fr/app_img/conform.svg"),
            FicheState::Accepted => embed = embed.image("https://intranet.projectvisualis.fr/app_img/accepted.svg"),
            FicheState::Refused => embed = embed.image("https://intranet.projectvisualis.fr/app_img/refused.svg"),
            FicheState::Comment => embed = embed.image("https://intranet.projectvisualis.fr/app_img/comment.svg"),
        }

        let builder = ExecuteWebhook::new()
            .embed(embed)
            .username("FicheRP");

        webhook.execute(&http, false, builder).await
    });
}