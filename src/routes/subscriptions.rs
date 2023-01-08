use axum::{debug_handler, extract::State, Form};
use http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[debug_handler]
pub async fn subscribe(State(pool): State<PgPool>, form: Form<FormData>) -> StatusCode {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(e) =>  {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        
    }
}
