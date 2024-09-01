use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::{doc, to_bson, Document};
use mongodb::Collection;
use serde::Deserialize;
use serenity::futures::TryStreamExt;
use std::time::Duration;
use uuid::Uuid;

use crate::utils::auth_utils::is_auth_valid;
use crate::{is_rate_limited, AppData};
use shared::fiche_rp::{FicheRP, FicheState, ReviewMessage};
use shared::permissions::DiscordRole;
use shared::user::FrontAccount;
use shared::website_meta::WebsiteMeta;

#[derive(Deserialize, Clone)]
struct FrontQuery {
    pub auth_id: String,
    pub fiche_id: Option<String>,
}

//TODO: FORCE PERMISSION CHECK

#[get("/api/front/retrieve_auth_account")]
pub async fn retrieve_auth_account(front_query: web::Query<FrontQuery>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*front_query.auth_id, app_data.dbclient.clone()).await {
        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

        let query = doc! {
            "auth_id" : &front_query.auth_id
        };
        let front_account: FrontAccount = accounts.find_one(query).await.expect("Can't retrieve accounts").unwrap();

        HttpResponse::Ok().json(&front_account)
    } else {
        HttpResponse::Unauthorized().body("")
    };
}

#[get("/api/front/retrieve_whitelist")]
pub async fn retrieve_whitelist(front_query: web::Query<FrontQuery>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*front_query.auth_id, app_data.dbclient.clone()).await {
        let accounts: Collection<WebsiteMeta> = app_data.dbclient.database("visualis-website").collection("website-meta");

        let meta: WebsiteMeta = accounts.find_one(Document::new()).await.expect("Can't retrieve accounts").unwrap();

        HttpResponse::Ok().json(&meta)
    } else {
        HttpResponse::Unauthorized().body("")
    };
}

#[post("/api/front/submit_ficherp")]
pub async fn submit_ficherp(front_query: web::Query<FrontQuery>, mut ficherp: web::Json<FicheRP>, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*front_query.auth_id, app_data.dbclient.clone()).await {
        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

        let query = doc! {
            "auth_id" : &front_query.auth_id
        };

        ficherp.id = Uuid::now_v7().to_string();
        ficherp.state = FicheState::Waiting;

        let update = doc! {
            "$push": { "fiches": to_bson(&ficherp.into_inner()).unwrap() }
        };

        match accounts.update_one(query, update).await {
            Ok(update_result) => {
                if update_result.matched_count > 0 {
                    HttpResponse::Ok().body("Fiche inserted successfully")
                } else {
                    HttpResponse::NotFound().body("Account not found")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to update account"),
        }
    } else {
        HttpResponse::Unauthorized().body("")
    };
}

#[post("/api/front/submit_ficherp_modif")]
pub async fn submit_ficherp_modif(front_query: web::Query<FrontQuery>, mut ficherp: web::Json<FicheRP>, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*front_query.auth_id, app_data.dbclient.clone()).await {
        let max_requests = 5;
        let time_window = Duration::from_secs(3600); // 1 hour

        // Call the rate limit function
        if is_rate_limited(&front_query.auth_id, max_requests, time_window, &app_data) {
            return HttpResponse::TooManyRequests().body("Rate limit exceeded. Try again later.");
        }

        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

        let query = doc! {
            "auth_id" : &front_query.auth_id,
            "fiches.id": &front_query.fiche_id
        };
        let update = doc! {
            "$set": {
                "fiches.$.state": to_bson(&FicheState::Waiting).unwrap(),
                "fiches.$.name": to_bson(&ficherp.name).unwrap(),
                "fiches.$.job": to_bson(&ficherp.job).unwrap(),
                "fiches.$.description": to_bson(&ficherp.description).unwrap(),
                "fiches.$.lore": to_bson(&ficherp.lore).unwrap(),
                "fiches.$.version": to_bson(&ficherp.version).unwrap(),
            }
        };

        match accounts.update_one(query, update).await {
            Ok(update_result) => {
                if update_result.matched_count > 0 {
                    HttpResponse::Ok().body("Comment inserted successfully")
                } else {
                    HttpResponse::NotFound().body("Account not found")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to update account"),
        }
    } else {
        HttpResponse::Unauthorized().body("")
    };
}

#[post("/api/front/submit_comment")]
pub async fn submit_comment(front_query: web::Query<FrontQuery>, mut comment: web::Json<ReviewMessage>, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*front_query.auth_id, app_data.dbclient.clone()).await {
        let max_requests = 3;
        let time_window = Duration::from_secs(300); // 1 hour

        // Call the rate limit function
        if is_rate_limited(&front_query.auth_id, max_requests, time_window, &app_data) {
            return HttpResponse::TooManyRequests().body("Rate limit exceeded. Try again later.");
        }
        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");
        let meta: Collection<WebsiteMeta> = app_data.dbclient.database("visualis-website").collection("website-meta");

        let whitelist: Vec<String> = meta.find_one(Document::new()).await.expect("Can't retrieve accounts").unwrap().whitelist;

        let query = doc! {
            "auth_id" : &front_query.auth_id
        };

        let user_account = accounts.find_one(query).await.unwrap().expect("Can't retrieve user!");

        if let Some(roles) = DiscordRole::from_role_ids(&user_account.discord_roles) {
            if whitelist.contains(&user_account.discord_user.id) || roles.iter().filter(|user_role| { **user_role == DiscordRole::PlatformAdmin || **user_role == DiscordRole::Admin || **user_role == DiscordRole::LeadScenarist || **user_role == DiscordRole::Scenarist }).count() > 1 || user_account.fiches.iter().filter(|fiche| fiche.id.eq(&front_query.fiche_id.clone().unwrap())).count() > 1 {
                let query = doc! {
                    "fiches.id": &front_query.fiche_id
                };
                let update = if comment.set_state == FicheState::Comment {
                    doc! {
                        "$push": {"fiches.$.messages": to_bson(&comment.into_inner()).unwrap()}
                    }
                } else {
                    doc! {
                        "$set": {"fiches.$.state": to_bson(&comment.set_state).unwrap()},
                        "$push": {"fiches.$.messages": to_bson(&comment.into_inner()).unwrap()}
                    }
                };
                match accounts.update_one(query, update).await {
                    Ok(update_result) => {
                        if update_result.matched_count > 0 {
                            HttpResponse::Ok().body("Comment inserted successfully")
                        } else {
                            HttpResponse::NotFound().body("Account not found")
                        }
                    }
                    Err(_) => HttpResponse::InternalServerError().body("Failed to update account"),
                }
            } else {
                HttpResponse::Unauthorized().body("")
            }
        } else {
            HttpResponse::Unauthorized().body("")
        }
    } else {
        HttpResponse::Unauthorized().body("")
    };
}

#[get("/api/front/retrieve_accounts")]
pub async fn retrieve_accounts(front_query: web::Query<FrontQuery>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    return if is_auth_valid(&*front_query.auth_id, app_data.dbclient.clone()).await {
        let accounts: Collection<FrontAccount> = app_data.dbclient.database("visualis-website").collection("account");

        let vec_front_account: Vec<FrontAccount> = accounts.find(Document::new()).await.expect("Can't retrieve accounts").try_collect().await.expect("Can't set account into vec");

        HttpResponse::Ok().json(&vec_front_account)
    } else {
        HttpResponse::Unauthorized().body("")
    };
}