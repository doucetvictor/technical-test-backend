use actix_web::{web, App, HttpServer};

mod errors;
mod routes;

use crate::routes::compare::compare;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v1/compare", web::get().to(compare))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
