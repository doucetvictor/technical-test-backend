use actix_web::{web, App, HttpServer};
use env_logger::Builder;
use log::info;

mod config;
mod errors;
mod routes;

use crate::routes::compare::compare;
use crate::config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("config.toml").expect("Failed to load config");
    let client = reqwest::Client::new();

    Builder::new()
        .parse_filters(config.general.logging.as_str())
        .init();

    info!("Starting server");
    info!("Listening on {}", config.general.bind_address);
    info!("Python API URL: {}", config.general.python_api_url);
    info!("Timeout: {} seconds", config.general.timeout);

    let bind_address = config.general.bind_address.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(config.clone()))
            .route("/v1/compare", web::get().to(compare))
    })
    .bind(bind_address)?
    .run()
    .await
}
