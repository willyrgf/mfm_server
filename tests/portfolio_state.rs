use serde::{Deserialize, Serialize};

mod common;

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    token_id: uuid::Uuid,
    rebalancer_label: String,
    data: String,
}

#[actix_web::test]
async fn portfolio_state_a_200_for_valid_json_body() {
    let app = common::spawn_app().await;

    let body = Body {
        token_id: uuid::Uuid::new_v4(),
        rebalancer_label: "label1".to_string(),
        data: r#"
            {"test": "{"inner": "aaaa"}""}
            "#
        .to_string(),
    };

    let string_body = serde_json::to_string(&body).unwrap();

    let response = {
        let client = reqwest::Client::new();

        client
            .post(&format!("{}/portfolio_state", app.address))
            .header("Content-Type", "application/json")
            .body(string_body)
            .send()
            .await
            .expect("failed to execute the request")
    };

    assert_eq!(response.status().as_u16(), 200);
    let saved = sqlx::query!(
        r#"
            select
                id, token_id, rebalancer_label
            from portfolio_states
            where true
            and token_id = $1
            and rebalancer_label = $2
            order by created_at desc
            limit 1
        "#,
        body.token_id,
        body.rebalancer_label
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed on fetch saved portfolio_state");

    assert_eq!(saved.token_id, body.token_id);
}

#[actix_web::test]
async fn portfolio_state_a_400_for_invalid_json_body() {
    struct BodyAndMessage {
        body: String,
        message: String,
    }

    let app = common::spawn_app().await;

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
            .post(&format!("{}/portfolio_state", app.address))
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
