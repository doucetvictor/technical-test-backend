use actix_web::{
    error,
    http::StatusCode,
    HttpResponse,
};
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    InternalError,
    BadClientData(String),
    Timeout,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MyError {}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::BadClientData(msg) => HttpResponse::BadRequest().body(msg.clone()),
            _ => HttpResponse::build(self.status_code()).finish(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData(_) => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
