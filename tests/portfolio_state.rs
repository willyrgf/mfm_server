use mfm_server::authentication::API_TOKEN_HEADER;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod common;

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    rebalancer_label: String,
    data: String,
}

#[actix_web::test]
async fn portfolio_state_a_200_for_valid_json_body() {
    let app = common::spawn_app().await;

    let token_label = format!("{}_test", Uuid::new_v4());
    let auth_token = sqlx::query!(
        r#"
        insert into auth_tokens (token_label)
        values ($1)
        returning token
        "#,
        token_label
    )
    .fetch_one(&app.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })
    .unwrap();

    let body = Body {
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
            .header(API_TOKEN_HEADER, auth_token.token.unwrap().to_string())
            .body(string_body)
            .send()
            .await
            .expect("failed to execute the request")
    };

    assert_eq!(response.status().as_u16(), 200);
    let saved = sqlx::query!(
        r#"
            select
                id, rebalancer_label
            from portfolio_states
            where true
            and rebalancer_label = $1
            order by created_at desc
            limit 1
        "#,
        body.rebalancer_label
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed on fetch saved portfolio_state");

    assert_eq!(saved.rebalancer_label, body.rebalancer_label);
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
                    "data": {"test": "aaa"}
                }
            "#
            .to_string(),
            message: "missing rebalancer_label".to_string(),
        },
        BodyAndMessage {
            body: r#"
                {
                    "rebalancer_label": "label1"
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

#[actix_web::test]
async fn portfolio_state_a_401_for_unathorized_access() {
    let app = common::spawn_app().await;
    let test_cases = vec![
        (
            Body {
                rebalancer_label: "label1".to_string(),
                data: r#"{"test": {"inner": "aaaa"}}"#.to_string(),
            },
            Uuid::new_v4().to_string(),
        ),
        (
            Body {
                rebalancer_label: "label1".to_string(),
                data: r#"{"test": {"inner": "aaaa"}}"#.to_string(),
            },
            "non_uuid_format".to_string(),
        ),
        (
            Body {
                rebalancer_label: "label1".to_string(),
                data: r#"{"test": {"inner": "aaaa"}}"#.to_string(),
            },
            "".to_string(),
        ),
    ];

    let client = reqwest::Client::new();

    for (body, api_token) in test_cases {
        let string_body = serde_json::to_string(&body).unwrap();

        let response = client
            .post(&format!("{}/portfolio_state", app.address))
            .header("Content-Type", "application/json")
            .header(API_TOKEN_HEADER, api_token)
            .body(string_body)
            .send()
            .await
            .expect("failed to execute the request");

        assert_eq!(
            response.status().as_u16(),
            401,
            "the API did not fail with 401"
        );
    }
}
