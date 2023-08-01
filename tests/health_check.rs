// tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It saves you from having to specify the `#[test]` attribute

// `cargo expand --test healt_check` will help you to inspect the code

#[tokio::test]

async fn healt_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn your app");

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
async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}
