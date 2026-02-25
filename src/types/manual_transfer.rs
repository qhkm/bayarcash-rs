use serde::Serialize;
use std::collections::HashMap;

/// Manual bank transfer request
#[derive(Debug, Clone, Serialize)]
pub struct ManualBankTransferRequest {
    pub portal_key: String,
    pub buyer_name: String,
    pub buyer_email: String,
    pub order_amount: f64,
    pub order_no: String,
    /// Must be 2 for manual transfers
    pub payment_gateway: u8,
    pub merchant_bank_name: String,
    pub merchant_bank_account: String,
    pub merchant_bank_account_holder: String,
    pub bank_transfer_type: String,
    pub bank_transfer_notes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_date: Option<String>,
    /// File path for proof of payment (server-side only)
    #[serde(skip)]
    pub proof_of_payment_path: Option<String>,
}

/// Parsed manual bank transfer HTML response
#[derive(Debug, Clone, Default)]
pub struct ManualBankTransferFormData {
    pub form_id: Option<String>,
    pub return_url: Option<String>,
    pub fields: HashMap<String, String>,
}

/// Manual bank transfer response
#[derive(Debug, Clone)]
pub enum ManualBankTransferResponse {
    Form {
        html_form: String,
        form_data: ManualBankTransferFormData,
        return_url: Option<String>,
    },
    Redirect {
        redirect_url: String,
    },
    Json(serde_json::Value),
}
