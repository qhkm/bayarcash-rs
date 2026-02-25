use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Payment channel identifiers matching Bayarcash API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentChannel {
    Fpx = 1,
    ManualTransfer = 2,
    FpxDirectDebit = 3,
    FpxLineOfCredit = 4,
    DuitnowDobw = 5,
    DuitnowQr = 6,
    Spaylater = 7,
    BoostPayflex = 8,
    Qrisob = 9,
    Qriswallet = 10,
    Nets = 11,
    CreditCard = 12,
    Alipay = 13,
    Wechatpay = 14,
    Promptpay = 15,
}

/// API version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApiVersion {
    #[default]
    V2,
    V3,
}

/// Request to create a payment intent
#[derive(Debug, Clone, Serialize)]
pub struct PaymentIntentRequest {
    pub payment_channel: u8,
    pub order_number: String,
    pub amount: f64,
    pub payer_name: String,
    pub payer_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_telephone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

/// Response from payment intent creation or retrieval
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub payer_name: String,
    pub payer_email: String,
    pub payer_telephone_number: Option<String>,
    pub order_number: String,
    pub amount: f64,
    pub url: String,
    #[serde(rename = "type")]
    pub intent_type: String,
    pub status: String,
    pub last_attempt: Option<Value>,
    pub paid_at: Option<String>,
    pub currency: String,
    #[serde(default)]
    pub attempts: Vec<Value>,
}
