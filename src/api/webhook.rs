use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Query;
use serde::Deserialize;
use serenity::all::{ExecuteWebhook, Http, Webhook};

#[derive(Deserialize)]
pub struct WebhookQuery {
    //token:String,
    webhook_id: String,
    webhook_token: String,
    webhook_name: String,
    content: String,
}

//TODO: auth
#[get("/api/sendDiscordWebhook")]
pub async fn send_discord_webhook(req: Query<WebhookQuery>) -> impl Responder {
    let http = Http::new("");
    let url = format!("https://discord.com/api/webhooks/{}/{}", req.webhook_id, req.webhook_token);

    let webhook = match Webhook::from_url(&http, url.as_str()).await {
        Ok(webhook) => webhook,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error resolving webhook: {}", err)),
    };

    let builder = ExecuteWebhook::new().content(req.content.as_str()).username(req.webhook_name.as_str());
    match webhook.execute(&http, false, builder).await {
        Ok(_) => HttpResponse::Ok().body("status:sent"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error executing webhook: {}", err)),
    }
}
