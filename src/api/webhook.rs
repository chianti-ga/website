use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::Form;
use serde::Deserialize;
use serenity::all::{Colour, CreateEmbed, ExecuteWebhook, Http, Webhook};

use crate::CONFIG;
use crate::config_utils::WebhookEntry;

#[derive(Deserialize)]
pub struct WebhookForm {
    webhook: String,
    content: String,
}
#[derive(Clone)]
#[derive(Deserialize)]
//FIXME: Wrong type on some field, not important for now because unused
pub struct EmbedWebhookForm {
    webhook: String,
    content: String,
    title: Option<String>,
    description: String,
    url: Option<String>,
    colour: Option<u32>,
    footer: Option<String>,
    image: Option<String>,
    thumbnail: Option<String>,
    author: Option<String>,
    fields: Option<String>,
}

#[get("/api/discord/text_webhook/")]
pub async fn text_webhook(form: Form<WebhookForm>) -> impl Responder {
    let choosed_webhook: &WebhookEntry = match CONFIG.webhooks_list.iter().filter(|x| x.webhook.to_lowercase() == form.webhook).next() {
        Some(webhook_entry) => webhook_entry,
        None => return HttpResponse::InternalServerError().body("webhook not found")
    };

    let http = Http::new("");
    let webhook = match Webhook::from_url(&http, choosed_webhook.url.as_str()).await {
        Ok(webhook) => webhook,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error building webhook: {}", err)),
    };

    let builder = ExecuteWebhook::new().content(form.content.as_str()).username(choosed_webhook.name.as_str());
    match webhook.execute(&http, false, builder).await {
        Ok(_) => HttpResponse::Ok().body("status:sent"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error executing webhook: {}", err)),
    }
}
//FIXME: Usable but not finished
#[post("/api/discord/embed_webhook/")]
pub async fn embed_webhook(form: Form<EmbedWebhookForm>) -> HttpResponse {
    let choosed_webhook: &WebhookEntry = match CONFIG.webhooks_list.iter().filter(|x| x.webhook.to_lowercase() == form.webhook).next() {
        Some(webhook_entry) => webhook_entry,
        None => return HttpResponse::BadRequest().body("webhook not found")
    };

    if form.description.len() > 4096 {
        return HttpResponse::PayloadTooLarge().body("description too large (>4096 characters).");
    }

    let http = Http::new("");

    let webhook = match Webhook::from_url(&http, choosed_webhook.url.as_str()).await {
        Ok(webhook) => webhook,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error building webhook: {}", err)),
    };

    let embed = CreateEmbed::new()
        .description(&form.clone().description)
        .title(&form.clone().title.unwrap_or_default())
        .url(&form.clone().url.unwrap_or_default())
        .colour(Colour::new(form.clone().colour.unwrap_or_default()))
        //.footer(&form.clone().footer.unwrap_or_default())
        .image(&form.clone().image.unwrap_or_default())
        .thumbnail(&form.clone().thumbnail.unwrap_or_default());
    //.author(&form.clone().author.unwrap_or_default())
    //.fields(&form.clone().fields.unwrap_or_default());

    let builder = ExecuteWebhook::new()
        .embed(embed)
        .username(choosed_webhook.name.as_str());

    match webhook.execute(&http, false, builder).await {
        Ok(_) => HttpResponse::Ok().body("status:sent"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error executing webhook: {}", err)),
    }
}
