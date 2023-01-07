use std::net::TcpListener;

use zero2prod::{build_router, run};

fn spawn_app() -> String {
    let router = build_router();
    let listener =  TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(run(router, listener));

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("failed to get reequest");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
