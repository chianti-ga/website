use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebhookQuery {
    acces_token: String,
    name: String,
    surname: String,
    id: String,
}

//TODO: auth
#[get("/api/getFicheRP")]
pub async fn get_fiche_rp(req: Query<WebhookQuery>) -> impl Responder {
    let octocrab = octocrab::Octocrab::builder()
        .user_access_token(req.acces_token.clone())
        .build().unwrap();
    HttpResponse::Ok().body("status:sent")
}