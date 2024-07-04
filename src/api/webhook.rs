use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Query;
use serde::Deserialize;
use serenity::all::{Embed, ExecuteWebhook, Http, Webhook};

use crate::CONFIG;
use crate::config_utils::TextWebhookEntry;

#[derive(Deserialize)]
pub struct WebhookQuery {
    webhook: String,
    content: String
}
pub struct EmbedWebhookData {
    embed: Embed,
}

#[get("/api/discord/text_webhook/")]
pub async fn text_webhook(req: Query<WebhookQuery>) -> impl Responder {
    let choosed_webhook: &TextWebhookEntry = match CONFIG.text_webhooks.iter().filter(|x| x.webhook.to_lowercase() == req.webhook).next() {
        Some(webhook_entry) => webhook_entry,
        None => return HttpResponse::InternalServerError().body("webhook not found")
    };

    let http = Http::new("");
    let webhook = match Webhook::from_url(&http, choosed_webhook.url.as_str()).await {
        Ok(webhook) => webhook,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error building webhook: {}", err)),
    };

    let builder = ExecuteWebhook::new().content(req.content.as_str()).username(choosed_webhook.name.as_str());
    match webhook.execute(&http, false, builder).await {
        Ok(_) => HttpResponse::Ok().body("status:sent"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error executing webhook: {}", err)),
    }
}

/*async fn send_text_webhook(webhook: Webhook, webhook_data: WebhookData) -> HttpResponse {
    let builder = ExecuteWebhook::new().content(webhook_data.content.as_str()).username(webhook_data.webhook_name.as_str());
    match webhook.execute(&Http::new(""), false, builder).await {
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error executing webhook: {}", err)),
    }
}*/
