use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Transaction response from API
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub id: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub datetime: Option<String>,
    pub payer_name: Option<String>,
    pub payer_email: Option<String>,
    pub payer_telephone_number: Option<String>,
    pub order_number: Option<String>,
    pub currency: Option<String>,
    pub amount: Option<f64>,
    pub exchange_reference_number: Option<String>,
    pub exchange_transaction_id: Option<String>,
    pub payer_bank_name: Option<String>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub return_url: Option<String>,
    pub metadata: Option<Value>,
    pub payout: Option<Value>,
    pub payment_gateway: Option<Value>,
    pub portal: Option<String>,
    pub merchant: Option<Value>,
    pub mandate: Option<Value>,
}

/// Query parameters for listing transactions (v3)
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransactionQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_channel: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_reference_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_email: Option<String>,
}

/// Pagination metadata
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaginationMeta {
    pub current_page: Option<u32>,
    pub from: Option<u32>,
    pub last_page: Option<u32>,
    pub per_page: Option<u32>,
    pub to: Option<u32>,
    pub total: Option<u32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Paginated transaction response
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub meta: PaginationMeta,
}
