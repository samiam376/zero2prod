use axum::{
    routing::{get, post},
    Router,
};
use http::header::HeaderName;
use sqlx::PgPool;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::info;

use crate::routes::{health_check::health, subscriptions::subscribe};

pub fn build_router(connection_pool: PgPool) -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");

    Router::new()
        .route("/health_check", get(health))
        .route("/subscriptions", post(subscribe))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        )
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
        .with_state(connection_pool)
}

pub async fn run(router: Router, listener: std::net::TcpListener) {
    let addr = listener.local_addr().unwrap();
    info!("listening on: {}", addr);

    axum::Server::from_tcp(listener)
        .expect("failed to build server")
        .serve(router.into_make_service())
        .await
        .unwrap();
}
