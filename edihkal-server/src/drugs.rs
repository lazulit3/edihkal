use axum::{http::StatusCode, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Drug {
    name: String,
}

pub async fn get_drugs() -> StatusCode {
    StatusCode::OK
}

pub async fn define_drug(Json(_payload): Json<Drug>) -> StatusCode {
    StatusCode::OK
}
