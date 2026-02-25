use bayarcash::{ApiVersion, Bayarcash, BayarcashError, PaymentChannel, PaymentIntentRequest};
use serde_json::json;

#[tokio::test]
async fn test_create_payment_intent() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/payment-intents")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "pi_123",
                "payer_name": "John Doe",
                "payer_email": "john@example.com",
                "order_number": "ORDER123",
                "amount": 100.50,
                "url": "https://pay.bayar.cash/redirect",
                "type": "payment_intent",
                "status": "pending",
                "currency": "MYR",
                "attempts": []
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = Bayarcash::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let request = PaymentIntentRequest {
        payment_channel: PaymentChannel::Fpx as u8,
        order_number: "ORDER123".to_string(),
        amount: 100.50,
        payer_name: "John Doe".to_string(),
        payer_email: "john@example.com".to_string(),
        payer_telephone_number: None,
        currency: None,
        callback_url: None,
        return_url: None,
        metadata: None,
        checksum: None,
    };

    let result = client.create_payment_intent(&request).await.unwrap();
    assert_eq!(result.id, "pi_123");
    assert_eq!(result.status, "pending");
    assert_eq!(result.url, "https://pay.bayar.cash/redirect");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_404() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/transactions/nonexistent")
        .with_status(404)
        .with_body("{}")
        .create_async()
        .await;

    let client = Bayarcash::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let result = client.get_transaction("nonexistent").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BayarcashError::NotFound));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_v3_method_on_v2_returns_error() {
    // No server needed - this check happens before any HTTP call
    let client = Bayarcash::builder("test_token")
        .api_version(ApiVersion::V2)
        .build_with_base_url("http://localhost:1234")
        .unwrap();

    let result = client.get_payment_intent("pi_123").await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BayarcashError::ApiVersionMismatch(method) => assert_eq!(method, "get_payment_intent"),
        other => panic!("Expected ApiVersionMismatch, got {:?}", other),
    }
}

#[tokio::test]
async fn test_fpx_banks_list() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/banks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {"id": "1", "name": "Maybank", "code": "MBB", "status": "active"},
                {"id": "2", "name": "CIMB", "code": "CIMB", "status": "active"}
            ])
            .to_string(),
        )
        .create_async()
        .await;

    let client = Bayarcash::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let banks = client.fpx_banks_list().await.unwrap();
    assert_eq!(banks.len(), 2);
    assert_eq!(banks[0].name, "Maybank");
    assert_eq!(banks[1].code, "CIMB");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_validation_error_422() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/payment-intents")
        .with_status(422)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "The given data was invalid.",
                "errors": {
                    "amount": ["The amount must be greater than 0."],
                    "payer_email": ["The payer email field is required."]
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = Bayarcash::builder("test_token")
        .build_with_base_url(&server.url())
        .unwrap();

    let request = PaymentIntentRequest {
        payment_channel: 1,
        order_number: "ORD1".to_string(),
        amount: 0.0,
        payer_name: "Test".to_string(),
        payer_email: "".to_string(),
        payer_telephone_number: None,
        currency: None,
        callback_url: None,
        return_url: None,
        metadata: None,
        checksum: None,
    };

    let result = client.create_payment_intent(&request).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BayarcashError::Validation { message, errors } => {
            assert!(message.contains("invalid"));
            assert!(errors.contains_key("amount"));
            assert!(errors.contains_key("payer_email"));
        }
        other => panic!("Expected Validation, got {:?}", other),
    }
    mock.assert_async().await;
}
