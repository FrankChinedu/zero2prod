use crate::helpers::get_address_and_client;

#[tokio::test]
async fn health_check_works() {
    let (test_app, client) = get_address_and_client().await;
    let response = client
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
