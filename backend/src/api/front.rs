use actix_session::Session;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use serde::Deserialize;
use serenity::futures::TryStreamExt;

use shared::user::FrontAccount;
use crate::AppData;
use crate::utils::auth_utils::is_auth_valid;

#[derive(Deserialize, Clone)]
struct Auth {
    pub auth_id: String,
}
#[get("/api/front/retrieve_auth_account")]
pub async fn retrieve_auth_account(auth: web::Query<Auth>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*auth.auth_id, app_data.dbclient.clone()).await {
        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

        let query = doc! {
            "auth_id" : &auth.auth_id
        };
        let front_account: FrontAccount = accounts.find_one(query).await.expect("Can't retrieve accounts").unwrap();

        HttpResponse::Ok().json(&front_account)
    } else {
        HttpResponse::Unauthorized().body("")
    }
}
#[get("/api/front/retrieve_accounts")]
pub async fn retrieve_accounts(auth: web::Query<Auth>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*auth.auth_id, app_data.dbclient.clone()).await {
        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

        let vec_front_account: Vec<FrontAccount> = accounts.find(Document::new()).await.expect("Can't retrieve accounts").try_collect().await.expect("Can't set account into vec");

        HttpResponse::Ok().json(&vec_front_account)
    } else {
        HttpResponse::Unauthorized().body("")
    }
}