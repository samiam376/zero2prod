use std::net::TcpListener;

use zero2prod::{
    configuration::get_configuration,
    startup::{build_router, run},
};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let router = build_router();
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind");

    run(router, listener).await;
}
