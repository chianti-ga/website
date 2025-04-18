use std::fs::create_dir_all;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use actix_cors::Cors;
use actix_files::Files;
use actix_session::config::CookieContentSecurity;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::http::header::HeaderName;
use actix_web::middleware::{Compress, Logger};
use actix_web::rt::time;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use config::{Config, File};
use dashmap::DashMap;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{error, info, warn};
use mongodb::bson::{bson, doc, Document};
use mongodb::{Collection, Cursor};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::{AuthUrl, Client, ClientId, ClientSecret, RedirectUrl, StandardRevocableToken, TokenResponse, TokenUrl};
use serenity::futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

use shared::user::Account;

use crate::api::front::{retrieve_accounts, retrieve_auth_account, retrieve_whitelist, submit_comment, submit_ficherp, submit_ficherp_admin, submit_ficherp_modif};
use crate::api::oauth2::{auth, callback};
use crate::utils::auth_utils::{renew_token, update_account_discord, update_auth_id};
use crate::utils::config_utils::{Configuration, Oauth2Client};

mod api;
mod utils;

lazy_static! {
     pub static ref CONFIG: Configuration = Config::builder().add_source(File::with_name("data/config.json")).build().expect("[ERROR] config.json not found or invalid.").try_deserialize::<Configuration>().unwrap();
}

struct AppData {
    client_map: DashMap<String, Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse>>,
    dbclient: mongodb::Client,
    reqwest_client: reqwest::Client,
    rate_limit_map: DashMap<String, RateLimitData>,
}

pub struct RateLimitData {
    request_count: usize,
    first_request_time: Instant,
}

#[actix_web::main]
async fn main() -> Result<()> {
    const GIT_COMMIT: Option<&str> = option_env!("GIT_COMMIT");
    const GIT_BRANCH: Option<&str> = option_env!("GIT_BRANCH");
    const BUILD_TIMESTAMP: Option<&str> = option_env!("BUILD_TIMESTAMP");
    const GIT_TAG: Option<&str> = option_env!("GIT_TAG");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting backend version {} on branch {} and compiled at {}", GIT_TAG.unwrap_or_else(|| "unknown"), GIT_BRANCH.unwrap_or_else(|| "unknown"), BUILD_TIMESTAMP.unwrap_or_else(|| "unknown"));

    match create_dir_all("data/cache/avatars") {
        Ok(_) => info!("Created cache folder for avatars"),
        Err(err) => error!("Can't create cache folder for avatars :{}",err)
    }

    let dbclient: mongodb::Client = init_mongo().await;

    let app_data = Data::new(AppData {
        client_map: DashMap::new(),
        dbclient: dbclient.clone(),
        reqwest_client: reqwest::Client::new(),
        rate_limit_map: Default::default(),
    });

    update_token_thread(dbclient.clone(), app_data.reqwest_client.clone()).await;
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // Global middlewares
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_origin("http://localhost:2828")
                .allow_any_method()
                .allow_any_header()
                .max_age(None)
            )
            .service(retrieve_accounts)
            .service(retrieve_auth_account)
            .service(submit_ficherp)
            .service(submit_ficherp_admin)
            .service(submit_comment)
            .service(submit_ficherp_modif)
            .service(retrieve_whitelist)
            .wrap({
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .build()
            })
            .wrap(Compress::default())
            .service(auth)
            .service(callback)
            .service(Files::new("/api/cache/avatars", "data/cache/avatars").index_file("index.html"))
            .service(Files::new("/", "dist").index_file("index.html"))
            .app_data(app_data.clone())
    })
        .bind((CONFIG.address.clone(), CONFIG.port))
        .map_err(anyhow::Error::msg)?
        .run()
        .await?;
    Ok(())
}

pub async fn init_mongo() -> mongodb::Client {
    let uri: &String = &CONFIG.mongo_db_uri;
    mongodb::Client::with_uri_str(uri).await.expect("[ERROR] Can't connect to mongodb server!")
}

async fn update_token_thread(dbclient: mongodb::Client, http_client: reqwest::Client) {
    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600)); // check every hour (3600s)
        let oauth2_info: &Oauth2Client = &CONFIG.oauth2client.clone();
        //IMPORTANT: The urls should NOT have "/" appended to the end, the lib will crash if so
        let oauth_client =
            BasicClient::new(
                ClientId::new(oauth2_info.client_id.clone()),
                Some(ClientSecret::new(oauth2_info.client_secret.clone())),
                AuthUrl::new(oauth2_info.auth_url.clone()).unwrap(),
                Some(TokenUrl::new(oauth2_info.token_url.clone()).unwrap()))
                // Set the URL the user will be redirected to after the authorization process.
                .set_redirect_uri(RedirectUrl::new(oauth2_info.redirect_url.clone()).unwrap());
        loop {
            interval.tick().await;

            let account_collection: Collection<Account> = dbclient.clone().database("visualis-website").collection("account");
            let mut accounts_cursor: Cursor<Account> = account_collection.find(Document::new()).await.expect("Can't get all account");

            while let Some(mut account) = accounts_cursor.try_next().await.expect("Can't iterate over collection") {
                let time_passed_since_renew: i64 = (account.last_renewal + account.token.expires_in().unwrap().as_secs()) as i64 - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);

                if time_passed_since_renew <= 0 {
                    warn!("Can't renew token for {}({}) since it has expired", account.discord_user.global_name, account.discord_user.id);
                    update_auth_id(&account.discord_user.id, &Uuid::now_v7().to_string(), dbclient.clone()).await;
                } else if time_passed_since_renew <= 86400 { //renew when one day or less is left
                    info!("Renewing token for {}({})", account.discord_user.global_name, account.discord_user.id);
                    renew_token(account.token.access_token().secret(), account.token.refresh_token().unwrap(), dbclient.clone(), oauth_client.clone()).await;
                } else {
                    update_account_discord(&account.auth_id, dbclient.clone(), &http_client).await;
                }
            }
        }
    });
}

pub fn is_rate_limited(
    auth_id: &str,
    max_requests: usize,
    time_window: Duration,
    app_data: &Data<AppData>,
) -> bool {
    let mut rate_limit_data = app_data.rate_limit_map.entry(auth_id.to_string()).or_insert_with(|| RateLimitData {
        request_count: 0,
        first_request_time: Instant::now(),
    });

    if rate_limit_data.first_request_time.elapsed() < time_window {
        if rate_limit_data.request_count >= max_requests {
            true
        } else {
            rate_limit_data.request_count += 1;
            false
        }
    } else {
        // Reset the counter if the time window has passed
        rate_limit_data.request_count = 1;
        rate_limit_data.first_request_time = Instant::now();
        false
    }
}