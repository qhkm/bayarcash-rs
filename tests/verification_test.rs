use bayarcash::checksum;
use bayarcash::types::*;
use bayarcash::verification;
use std::collections::BTreeMap;

#[test]
fn test_verify_transaction_callback_valid() {
    let secret = "test_secret_key_12345";
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), "100.50".to_string());
    fields.insert("currency".to_string(), "MYR".to_string());
    fields.insert("datetime".to_string(), "2025-01-15 10:30:00".to_string());
    fields.insert(
        "exchange_reference_number".to_string(),
        "REF123".to_string(),
    );
    fields.insert("exchange_transaction_id".to_string(), "EXT123".to_string());
    fields.insert("order_number".to_string(), "ORDER123".to_string());
    fields.insert("payer_bank_name".to_string(), "Maybank".to_string());
    fields.insert("payer_email".to_string(), "john@example.com".to_string());
    fields.insert("payer_name".to_string(), "John Doe".to_string());
    fields.insert("record_type".to_string(), "transaction".to_string());
    fields.insert("status".to_string(), "3".to_string());
    fields.insert("status_description".to_string(), "Successful".to_string());
    fields.insert("transaction_id".to_string(), "TXN123".to_string());
    let valid_checksum = checksum::create_checksum_value(secret, &fields);

    let data = TransactionCallbackData {
        record_type: "transaction".into(),
        transaction_id: "TXN123".into(),
        exchange_reference_number: "REF123".into(),
        exchange_transaction_id: "EXT123".into(),
        order_number: "ORDER123".into(),
        currency: "MYR".into(),
        amount: "100.50".into(),
        payer_name: "John Doe".into(),
        payer_email: "john@example.com".into(),
        payer_bank_name: "Maybank".into(),
        status: "3".into(),
        status_description: "Successful".into(),
        datetime: "2025-01-15 10:30:00".into(),
        checksum: valid_checksum,
    };
    assert!(verification::verify_transaction(&data, secret));
}

#[test]
fn test_verify_transaction_callback_invalid() {
    let data = TransactionCallbackData {
        record_type: "transaction".into(),
        transaction_id: "TXN123".into(),
        exchange_reference_number: "REF123".into(),
        exchange_transaction_id: "EXT123".into(),
        order_number: "ORDER123".into(),
        currency: "MYR".into(),
        amount: "100.50".into(),
        payer_name: "John Doe".into(),
        payer_email: "john@example.com".into(),
        payer_bank_name: "Maybank".into(),
        status: "3".into(),
        status_description: "Successful".into(),
        datetime: "2025-01-15 10:30:00".into(),
        checksum: "invalid_checksum_here".into(),
    };
    assert!(!verification::verify_transaction(
        &data,
        "test_secret_key_12345"
    ));
}

#[test]
fn test_verify_pre_transaction_callback_valid() {
    let secret = "test_secret_key_12345";
    let mut fields = BTreeMap::new();
    fields.insert(
        "exchange_reference_number".to_string(),
        "REF123".to_string(),
    );
    fields.insert("order_number".to_string(), "ORDER123".to_string());
    fields.insert("record_type".to_string(), "pre_transaction".to_string());
    let valid_checksum = checksum::create_checksum_value(secret, &fields);

    let data = PreTransactionCallbackData {
        record_type: "pre_transaction".into(),
        exchange_reference_number: "REF123".into(),
        order_number: "ORDER123".into(),
        checksum: valid_checksum,
    };
    assert!(verification::verify_pre_transaction(&data, secret));
}

#[test]
fn test_verify_return_url_callback_valid() {
    let secret = "test_secret_key_12345";
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), "100.50".to_string());
    fields.insert("currency".to_string(), "MYR".to_string());
    fields.insert(
        "exchange_reference_number".to_string(),
        "REF123".to_string(),
    );
    fields.insert("exchange_transaction_id".to_string(), "EXT123".to_string());
    fields.insert("order_number".to_string(), "ORDER123".to_string());
    fields.insert("payer_bank_name".to_string(), "Maybank".to_string());
    fields.insert("status".to_string(), "3".to_string());
    fields.insert("status_description".to_string(), "Successful".to_string());
    fields.insert("transaction_id".to_string(), "TXN123".to_string());
    let valid_checksum = checksum::create_checksum_value(secret, &fields);

    let data = ReturnUrlCallbackData {
        transaction_id: "TXN123".into(),
        exchange_reference_number: "REF123".into(),
        exchange_transaction_id: "EXT123".into(),
        order_number: "ORDER123".into(),
        currency: "MYR".into(),
        amount: "100.50".into(),
        payer_bank_name: "Maybank".into(),
        status: "3".into(),
        status_description: "Successful".into(),
        checksum: valid_checksum,
    };
    assert!(verification::verify_return_url(&data, secret));
}
