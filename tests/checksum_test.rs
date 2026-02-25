use bayarcash_sdk::checksum;
use std::collections::BTreeMap;

#[test]
fn test_payment_intent_checksum_is_64_hex_chars() {
    let checksum = checksum::payment_intent(
        "test_secret_key_12345",
        1,
        "ORDER123",
        100.5,
        "John Doe",
        "john@example.com",
    );
    assert_eq!(checksum.len(), 64);
    assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_payment_intent_checksum_is_deterministic() {
    let c1 = checksum::payment_intent(
        "test_secret_key_12345",
        1,
        "ORDER123",
        100.5,
        "John Doe",
        "john@example.com",
    );
    let c2 = checksum::payment_intent(
        "test_secret_key_12345",
        1,
        "ORDER123",
        100.5,
        "John Doe",
        "john@example.com",
    );
    assert_eq!(c1, c2);
}

#[test]
fn test_payment_intent_checksum_differs_for_different_data() {
    let c1 = checksum::payment_intent(
        "test_secret_key_12345",
        1,
        "ORDER123",
        100.5,
        "John Doe",
        "john@example.com",
    );
    let c2 = checksum::payment_intent(
        "test_secret_key_12345",
        1,
        "ORDER456",
        100.5,
        "John Doe",
        "john@example.com",
    );
    assert_ne!(c1, c2);
}

#[test]
fn test_fpx_dd_enrollment_checksum_is_64_hex_chars() {
    let checksum = checksum::fpx_direct_debit_enrollment(
        "test_secret_key_12345",
        "DD-ORDER-001",
        50.0,
        "Jane Doe",
        "jane@example.com",
        "+60123456789",
        "NRIC",
        "920101015678",
        "Monthly subscription",
        "MONTHLY",
    );
    assert_eq!(checksum.len(), 64);
    assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_fpx_dd_maintenance_checksum_is_64_hex_chars() {
    let checksum = checksum::fpx_direct_debit_maintenance(
        "test_secret_key_12345",
        75.0,
        "jane@example.com",
        "+60123456789",
        "Update subscription amount",
        "MONTHLY",
    );
    assert_eq!(checksum.len(), 64);
    assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_generic_checksum_sorts_keys_alphabetically() {
    let mut fields1 = BTreeMap::new();
    fields1.insert("zebra".to_string(), "1".to_string());
    fields1.insert("alpha".to_string(), "2".to_string());

    let mut fields2 = BTreeMap::new();
    fields2.insert("alpha".to_string(), "2".to_string());
    fields2.insert("zebra".to_string(), "1".to_string());

    let c1 = checksum::create_checksum_value("secret", &fields1);
    let c2 = checksum::create_checksum_value("secret", &fields2);
    assert_eq!(c1, c2);
}
