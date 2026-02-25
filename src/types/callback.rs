use serde::Deserialize;

/// Transaction callback data from webhook
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionCallbackData {
    pub record_type: String,
    pub transaction_id: String,
    pub exchange_reference_number: String,
    pub exchange_transaction_id: String,
    pub order_number: String,
    pub currency: String,
    pub amount: String,
    pub payer_name: String,
    pub payer_email: String,
    pub payer_bank_name: String,
    pub status: String,
    pub status_description: String,
    pub datetime: String,
    pub checksum: String,
}

/// Pre-transaction callback data
#[derive(Debug, Clone, Deserialize)]
pub struct PreTransactionCallbackData {
    pub record_type: String,
    pub exchange_reference_number: String,
    pub order_number: String,
    pub checksum: String,
}

/// Return URL callback data
#[derive(Debug, Clone, Deserialize)]
pub struct ReturnUrlCallbackData {
    pub transaction_id: String,
    pub exchange_reference_number: String,
    pub exchange_transaction_id: String,
    pub order_number: String,
    pub currency: String,
    pub amount: String,
    pub payer_bank_name: String,
    pub status: String,
    pub status_description: String,
    pub checksum: String,
}

/// Direct Debit bank approval callback data
#[derive(Debug, Clone, Deserialize)]
pub struct DirectDebitBankApprovalCallbackData {
    pub record_type: String,
    pub approval_date: String,
    pub approval_status: String,
    pub mandate_id: String,
    pub mandate_reference_number: String,
    pub order_number: String,
    pub payer_bank_code_hashed: String,
    pub payer_bank_code: String,
    pub payer_bank_account_no: String,
    pub application_type: String,
    pub checksum: String,
}

/// Direct Debit authorization callback data
#[derive(Debug, Clone, Deserialize)]
pub struct DirectDebitAuthorizationCallbackData {
    pub record_type: String,
    pub transaction_id: String,
    pub mandate_id: String,
    pub exchange_reference_number: String,
    pub exchange_transaction_id: String,
    pub order_number: String,
    pub currency: String,
    pub amount: String,
    pub payer_name: String,
    pub payer_email: String,
    pub payer_bank_name: String,
    pub status: String,
    pub status_description: String,
    pub datetime: String,
    pub checksum: String,
}

/// Direct Debit transaction callback data
#[derive(Debug, Clone, Deserialize)]
pub struct DirectDebitTransactionCallbackData {
    pub record_type: String,
    pub batch_number: String,
    pub mandate_id: String,
    pub mandate_reference_number: String,
    pub transaction_id: String,
    pub datetime: String,
    pub reference_number: String,
    pub amount: String,
    pub status: String,
    pub status_description: String,
    pub cycle: String,
    pub checksum: String,
}
