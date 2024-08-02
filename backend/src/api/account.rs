use actix_session::Session;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::http::header::LOCATION;
use mongodb::bson::doc;
use mongodb::Collection;
use serenity::futures::{StreamExt, TryStreamExt};

use shared::user::{Account, FrontAccount};

use crate::AppData;
use crate::utils::auth_utils::{is_auth_valid, update_account_discord};

#[get("/api/retrieve_accounts")]
pub async fn retrieve_accounts(req: HttpRequest, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    println!("{:?}", req.cookies().unwrap());
    if let Some(cookie) = req.cookie("auth_id") {
        if is_auth_valid(cookie.clone().value(), app_data.dbclient.clone()).await {
            let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

            let query = doc! {
                "auth_id" : cookie.value()
            };
            let vec_front_account: Vec<FrontAccount> = accounts.find(query).await.expect("Can't retrieve accounts").try_collect().await.expect("Can't set account into vec");

            return HttpResponse::Ok().body(serde_json::to_string(&vec_front_account).expect("cant serialize"));
        }
    } else {
        return HttpResponse::BadRequest().body("");
    }
    return HttpResponse::BadRequest().body("");
}