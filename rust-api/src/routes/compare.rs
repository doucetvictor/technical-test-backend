use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use chrono::NaiveDate;
use crate::errors::MyError;
use reqwest::Client;
use crate::config::Config;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct CompareParams {
    start_date: String,
    end_date: String,
}

pub async fn compare(
    params: web::Query<CompareParams>,
    client: web::Data<Client>,
    config: web::Data<Config>,
) -> Result<impl Responder, MyError> {
    let start_date = NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d")
        .map_err(|_| MyError::BadClientData("start_date is invalid".to_string()))?;

    let end_date = NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d")
        .map_err(|_| MyError::BadClientData("end_date is invalid".to_string()))?;

    if start_date > end_date {
        return Err(MyError::BadClientData("start_date is after end_date".to_string()));
    }

    let url = format!(
        "{}/v1/compare?start_date={}&end_date={}",
        config.general.python_api_url,
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );

    let response = client
        .get(url)
        .timeout(Duration::from_secs(config.general.timeout))
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                MyError::Timeout
            } else {
                MyError::InternalError(format!("Failed to call Python API: {}", e))
            }
        })?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|e| MyError::InternalError(format!("Failed to read body: {}", e)))?;
        Ok(HttpResponse::Ok().body(body))
    } else {
        Err(MyError::InternalError(format!("Python API returned error: {}", response.status())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_compare_valid() {
        let app = test::init_service(App::new().route("/v1/compare", web::get().to(compare))).await;
        let req = test::TestRequest::get()
            .uri("/v1/compare?start_date=2023-01-01&end_date=2023-01-31")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_compare_invalid_start_date() {
        let app = test::init_service(App::new().route("/v1/compare", web::get().to(compare))).await;
        let req = test::TestRequest::get()
            .uri("/v1/compare?start_date=invalid&end_date=2023-01-31")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, "start_date is invalid".as_bytes());
    }

    #[actix_web::test]
    async fn test_compare_invalid_end_date() {
        let app = test::init_service(App::new().route("/v1/compare", web::get().to(compare))).await;
        let req = test::TestRequest::get()
            .uri("/v1/compare?start_date=2023-01-01&end_date=invalid")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, "end_date is invalid".as_bytes());
    }

    #[actix_web::test]
    async fn test_compare_start_after_end() {
        let app = test::init_service(App::new().route("/v1/compare", web::get().to(compare))).await;
        let req = test::TestRequest::get()
            .uri("/v1/compare?start_date=2023-02-01&end_date=2023-01-31")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
        let body = test::read_body(resp).await;
        assert_eq!(body, "start_date is after end_date".as_bytes());
    }
}
