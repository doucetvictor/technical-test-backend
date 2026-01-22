use actix_web::{HttpResponse, Responder};

pub async fn healthy() -> impl Responder {
    HttpResponse::Ok()
}
