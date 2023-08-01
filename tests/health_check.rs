// tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It saves you from having to specify the `#[test]` attribute

// `cargo expand --test healt_check` will help you to inspect the code

#[tokio::test]
async fn healt_check_works() {
    // Arrange
    spawn_app();

    // Bring `reqwest` to perform HTTP requests against
    // our application

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:9090/health_check")
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application
fn spawn_app() {
    // we will request for an available port
    // we will use Port 0 for this at OS level
    // This allows to bind to port 0 and in case of error
    // OS will bind the app to the next available random port

    let server = zero2prod::run("0.0.0.0:0").expect("Failed to bind address");

    // launch the server as a background task

    let _ = tokio::spawn(server);
}
