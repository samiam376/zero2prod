use std::{net::TcpListener};

use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    startup::{build_router, run},
};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&configuration.database.connection_string())
        .await
        .expect("can't connect to database");

    let router = build_router(pool);
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind");

    run(router, listener).await;
}
