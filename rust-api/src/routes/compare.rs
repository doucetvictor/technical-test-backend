use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use chrono::NaiveDate;
use crate::errors::MyError;

#[derive(Debug, Deserialize)]
pub struct CompareParams {
    start_date: String,
    end_date: String,
}

pub async fn compare(params: web::Query<CompareParams>) -> Result<impl Responder, MyError> {
    let start_date = NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d")
        .map_err(|_| MyError::BadClientData("start_date is invalid".to_string()))?;

    let end_date = NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d")
        .map_err(|_| MyError::BadClientData("end_date is invalid".to_string()))?;

    if start_date > end_date {
        return Err(MyError::BadClientData("start_date is after end_date".to_string()));
    }

    Ok(HttpResponse::Ok().body("Dates are valid"))
}
