use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// FPX Direct Debit enrollment request
#[derive(Debug, Clone, Serialize)]
pub struct FpxDirectDebitEnrollmentRequest {
    pub order_number: String,
    pub amount: f64,
    pub payer_name: String,
    pub payer_email: String,
    pub payer_telephone_number: String,
    pub payer_id_type: String,
    pub payer_id: String,
    pub application_reason: String,
    pub frequency_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

/// FPX Direct Debit maintenance request
#[derive(Debug, Clone, Serialize)]
pub struct FpxDirectDebitMaintenanceRequest {
    pub amount: f64,
    pub payer_email: String,
    pub payer_telephone_number: String,
    pub application_reason: String,
    pub frequency_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

/// FPX Direct Debit application response
#[derive(Debug, Clone, Deserialize)]
pub struct FpxDirectDebitApplication {
    pub id: String,
    pub url: String,
    pub status: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// FPX Direct Debit details response
#[derive(Debug, Clone, Deserialize)]
pub struct FpxDirectDebit {
    pub id: String,
    pub mandate_reference_number: Option<String>,
    pub status: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
