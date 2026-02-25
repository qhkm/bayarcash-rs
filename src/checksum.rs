use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fmt::Display;

type HmacSha256 = Hmac<Sha256>;

/// Generate HMAC-SHA256 checksum from sorted key-value pairs joined by pipe.
pub fn create_checksum_value(secret_key: &str, fields: &BTreeMap<String, String>) -> String {
    let payload_string: String = fields.values().cloned().collect::<Vec<_>>().join("|");
    let mut mac =
        HmacSha256::new_from_slice(secret_key.as_bytes()).expect("HMAC accepts any key length");
    mac.update(payload_string.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Create payment intent checksum
pub fn payment_intent(
    secret_key: &str,
    payment_channel: u8,
    order_number: &str,
    amount: impl Display,
    payer_name: &str,
    payer_email: &str,
) -> String {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), amount.to_string());
    fields.insert("order_number".to_string(), order_number.to_string());
    fields.insert("payer_email".to_string(), payer_email.to_string());
    fields.insert("payer_name".to_string(), payer_name.to_string());
    fields.insert("payment_channel".to_string(), payment_channel.to_string());
    create_checksum_value(secret_key, &fields)
}

/// Create FPX Direct Debit enrollment checksum
#[allow(clippy::too_many_arguments)]
pub fn fpx_direct_debit_enrollment(
    secret_key: &str,
    order_number: &str,
    amount: impl Display,
    payer_name: &str,
    payer_email: &str,
    payer_telephone_number: &str,
    payer_id_type: &str,
    payer_id: &str,
    application_reason: &str,
    frequency_mode: &str,
) -> String {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), amount.to_string());
    fields.insert(
        "application_reason".to_string(),
        application_reason.to_string(),
    );
    fields.insert("frequency_mode".to_string(), frequency_mode.to_string());
    fields.insert("order_number".to_string(), order_number.to_string());
    fields.insert("payer_email".to_string(), payer_email.to_string());
    fields.insert("payer_id".to_string(), payer_id.to_string());
    fields.insert("payer_id_type".to_string(), payer_id_type.to_string());
    fields.insert("payer_name".to_string(), payer_name.to_string());
    fields.insert(
        "payer_telephone_number".to_string(),
        payer_telephone_number.to_string(),
    );
    create_checksum_value(secret_key, &fields)
}

/// Create FPX Direct Debit maintenance checksum
pub fn fpx_direct_debit_maintenance(
    secret_key: &str,
    amount: impl Display,
    payer_email: &str,
    payer_telephone_number: &str,
    application_reason: &str,
    frequency_mode: &str,
) -> String {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), amount.to_string());
    fields.insert(
        "application_reason".to_string(),
        application_reason.to_string(),
    );
    fields.insert("frequency_mode".to_string(), frequency_mode.to_string());
    fields.insert("payer_email".to_string(), payer_email.to_string());
    fields.insert(
        "payer_telephone_number".to_string(),
        payer_telephone_number.to_string(),
    );
    create_checksum_value(secret_key, &fields)
}
