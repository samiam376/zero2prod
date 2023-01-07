use axum::Form;
use http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>) -> StatusCode {
    StatusCode::OK
}
