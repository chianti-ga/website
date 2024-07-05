use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use anyhow::Result;
use config::{Config, File};
use env_logger::Env;
use lazy_static::lazy_static;

use crate::api::webhook::{embed_webhook, text_webhook};
use crate::config_utils::Configuration;

mod api;
mod config_utils;


lazy_static! {
     pub static ref CONFIG: Configuration = Config::builder().add_source(File::with_name("config.json")).build().expect("[ERROR] config.json not found or invalid.").try_deserialize::<Configuration>().unwrap();
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(text_webhook)
            .service(embed_webhook)
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
