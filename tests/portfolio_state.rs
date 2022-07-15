mod common;

#[actix_web::test]
async fn portfolio_state_a_200_for_valid_json_body() {
    let addr = common::spawn_app();

    let response = {
        let client = reqwest::Client::new();
        let body = r#"
					{
						"token_id": "04a370dc-c864-453c-875a-bf00ee839ae7",
						"rebalancer_label": "label1",
						"data": {"test": "aaa"}
					}
				"#;

        client
            .post(&format!("{}/portfolio_state", &addr))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("failed to execute the request")
    };

    assert_eq!(response.status().as_u16(), 200);
}

#[actix_web::test]
async fn portfolio_state_a_400_for_invalid_json_body() {
    struct BodyAndMessage {
        body: String,
        message: String,
    }

    let addr = common::spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![
        BodyAndMessage {
            body: r#"
									{
										"token_id": "wrong-uuid-format-here",
										"rebalancer_label": "label1",
										"data": {"test": "aaa"}
									}
								"#
            .to_string(),
            message: "wrong uuid format in token_id".to_string(),
        },
        BodyAndMessage {
            body: r#"
									{
										"token_id": "04a370dc-c864-453c-875a-bf00ee839ae7",
										"data": {"test": "aaa"}
									}
								"#
            .to_string(),
            message: "missing rebalancer_label".to_string(),
        },
        BodyAndMessage {
            body: r#"
								{
									"token_id": "04a370dc-c864-453c-875a-bf00ee839ae7",
									"rebalancer_label": "label1",
								}
							"#
            .to_string(),
            message: "missing data".to_string(),
        },
    ];

    for body_and_message in test_cases {
        let response = client
            .post(&format!("{}/portfolio_state", &addr))
            .header("Content-Type", "application/json")
            .body(body_and_message.body)
            .send()
            .await
            .expect("failed to execute the request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "the API did not fail with 400 when payload was {}",
            body_and_message.message
        );
    }
}
