// tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It saves you from having to specify the `#[test]` attribute

// `cargo expand --test healt_check` will help you to inspect the code

use std::{fmt::format, net::TcpListener};

#[tokio::test]
async fn healt_check_works() {
    // Arrange
    let address = spawn_app();

    // Bring `reqwest` to perform HTTP requests against
    // our application

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application
fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");

    // retrieve the port assinged to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // launch the server as a background task

    let _ = tokio::spawn(server);
    format!("http://0.0.0.0:{}", port)
}
