use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use actix_cors::Cors;
use actix_files::Files;
use actix_session::config::CookieContentSecurity;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use actix_web::cookie::Key;
use actix_web::http::header::HeaderName;
use actix_web::middleware::Logger;
use actix_web::rt::time;
use actix_web::web::Data;
use anyhow::Result;
use config::{Config, File};
use dashmap::DashMap;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{error, info};
use mongodb::{Collection, Cursor};
use mongodb::bson::{bson, doc, Document};
use oauth2::{AuthUrl, Client, ClientId, ClientSecret, RedirectUrl, StandardRevocableToken, TokenResponse, TokenUrl};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use serenity::futures::{StreamExt, TryStreamExt};

use shared::user::Account;

use crate::api::front::{retrieve_accounts, retrieve_auth_account, submit_ficherp};
use crate::api::oauth2::{auth, callback};
use crate::api::webhook::{embed_webhook, text_webhook};
use crate::utils::auth_utils::renew_token;
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
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let dbclient: mongodb::Client = init_mongo().await;

    let app_data = Data::new(AppData {
        client_map: DashMap::new(),
        dbclient: dbclient.clone(),
        reqwest_client: reqwest::Client::new()

    });

    update_token_thread(dbclient.clone()).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_origin("http://localhost:2828")
                .allow_any_method()
                .allow_any_header()
                .max_age(None)
            )
            .wrap({
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .build()
            })
            .service(text_webhook)
            .service(embed_webhook)
            .service(auth)
            .service(callback)
            .service(retrieve_accounts)
            .service(retrieve_auth_account)
            .service(submit_ficherp)
            .service(Files::new("/", "dist").index_file("index.html"))
            .app_data(app_data.clone())
    })
        .bind(("127.0.0.1", CONFIG.port))
        .map_err(anyhow::Error::msg)?
        .run()
        .await?;
    Ok(())
}

pub async fn init_mongo() -> mongodb::Client {
    let uri: &String = &CONFIG.mongo_db_uri;
    mongodb::Client::with_uri_str(uri).await.expect("[ERROR] Can't connect to mongodb server!")
}

pub async fn update_token_thread(dbclient: mongodb::Client) {
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

            while let Some(account) = accounts_cursor.try_next().await.expect("Can't iterate over collection") {
                let time_passed_since_renew = (account.last_renewal + account.token.expires_in().unwrap().as_secs()) - SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if time_passed_since_renew <= 86400 { //renew when one day or less is left
                    renew_token(account.token.access_token().secret(), account.token.refresh_token().unwrap(), dbclient.clone(), oauth_client.clone()).await;
                    info!("Renewing token for {}({})", account.discord_user.username, account.discord_user.id);
                }
            }
        }
    });
}
