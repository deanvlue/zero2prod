// tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It saves you from having to specify the `#[test]` attribute

// `cargo expand --test healt_check` will help you to inspect the code

use std::net::TcpListener;

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act

    let body = "name=le%20guin&email=ursula_le_guin%40example.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40example.com", "missing the name"),
        ("", "missing both fields"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional cutomized error on test failure
            "The API failed with 400 Bad Request when payload was {}",
            error_message
        );
    }
}
