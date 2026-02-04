use crate::helpers::{spawn_app, TestApp};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let TestApp { address, .. } = spawn_app().await;
    // Act
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
    assert_eq!("OK", response.text().await.unwrap());
}
