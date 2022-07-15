mod common;

#[actix_web::test]
async fn health_check_works() {
    let addr = common::spawn_app();

    let response = {
        let client = reqwest::Client::new();

        client
            .get(&format!("{}/health_check", addr))
            .send()
            .await
            .expect("failed to execute the request")
    };

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.content_length(), Some(0));
}
