use std::collections::HashMap;
use std::sync::Mutex;

use actix_session::config::CookieContentSecurity;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use anyhow::Result;
use config::{Config, File};
use env_logger::Env;
use lazy_static::lazy_static;
use oauth2::{Client, StandardRevocableToken};
use oauth2::basic::{BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};

use crate::api::oauth2::{auth, callback};
use crate::api::webhook::{embed_webhook, text_webhook};
use crate::utils::config_utils::Configuration;
use crate::utils::database_utils::DatabaseStruct;

mod api;
mod utils;

lazy_static! {
     pub static ref CONFIG: Configuration = Config::builder().add_source(File::with_name("data/config.json")).build().expect("[ERROR] config.json not found or invalid.").try_deserialize::<Configuration>().unwrap();
}

struct AppData {
    client_map: Mutex<HashMap<String, Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse>>>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_data = Data::new(AppData {
        client_map: Mutex::new(HashMap::new()),
    });
    let database = DatabaseStruct::init().await;
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap({
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .build()
            })
            .app_data(database.clone())
            .app_data(app_data.clone())
            .service(hello)
            .service(text_webhook)
            .service(embed_webhook)
            .service(auth)
            .service(callback)
    })
        .bind(("127.0.0.1", CONFIG.port))
        .map_err(anyhow::Error::msg)?
        .run()
        .await?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(include_str!("ress/frontpage.html"))
}
