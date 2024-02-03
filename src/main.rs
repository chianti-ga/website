use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use anyhow::Result;
// Import the Result type from the anyhow crate
use config::Config;

use crate::api::webhook::send_discord_webhook;

mod api;

#[actix_web::main]
async fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing()?;

    color_eyre::install().unwrap(); // color backtracking

    let config = Config::builder()
        .add_source(config::File::with_name("config.json"))
        .build()
        .map_err(anyhow::Error::msg)?;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(send_discord_webhook)
    })
        .bind(("127.0.0.1", config.get("port").unwrap()))
        .map_err(anyhow::Error::msg)?
        .run()
        .await?;

    Ok(())
}

#[cfg(feature = "capture-spantrace")]
fn install_tracing() -> Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{EnvFilter, fmt};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .try_init()
        .map_err(anyhow::Error::msg)?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
