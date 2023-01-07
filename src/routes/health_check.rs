use http::StatusCode;

pub async fn health() -> StatusCode {
    StatusCode::OK
}
