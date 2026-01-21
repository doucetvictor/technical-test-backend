use actix_web::{web, App, HttpServer};

mod errors;
mod routes;

use crate::routes::compare::compare;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = reqwest::Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/v1/compare", web::get().to(compare))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
