use kadena::{ApiClient, ApiConfig, Cmd, FetchError};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_local_execution() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/chainweb/0.0/testnet04/chain/0/pact/api/v1/local"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"result": "success"})))
        .mount(&mock_server)
        .await;

    let cmd = Cmd {
        hash: "test_hash".to_string(),
        sigs: vec![],
        cmd: "test_cmd".to_string(),
    };

    let client = ApiClient::new(ApiConfig::new(&mock_server.uri(), "testnet04", "0"));
    let result = client.local(&cmd).await.unwrap();
    assert_eq!(result, json!({"result": "success"}));
}

#[tokio::test]
async fn test_send_transaction() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/chainweb/0.0/testnet04/chain/0/pact/api/v1/send"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"requestKeys": ["test_key"]})),
        )
        .mount(&mock_server)
        .await;

    let cmd = Cmd {
        hash: "test_hash".to_string(),
        sigs: vec![],
        cmd: "test_cmd".to_string(),
    };

    let client = ApiClient::new(ApiConfig::new(&mock_server.uri(), "testnet04", "0"));
    let result = client.send(&cmd).await.unwrap();
    assert_eq!(result, json!({"requestKeys": ["test_key"]}));
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/local"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Invalid request"))
        .mount(&mock_server)
        .await;

    let cmd = Cmd {
        hash: "test_hash".to_string(),
        sigs: vec![],
        cmd: "test_cmd".to_string(),
    };

    let client = ApiClient::new(ApiConfig::new(&mock_server.uri(), "testnet04", "0"));
    let result = client.local(&cmd).await;
    assert!(matches!(result, Err(FetchError::ApiError(_))));
}
