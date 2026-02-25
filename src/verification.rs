use crate::checksum::create_checksum_value;
use crate::types::*;
use std::collections::BTreeMap;

pub fn verify_transaction(data: &TransactionCallbackData, secret_key: &str) -> bool {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), data.amount.clone());
    fields.insert("currency".to_string(), data.currency.clone());
    fields.insert("datetime".to_string(), data.datetime.clone());
    fields.insert(
        "exchange_reference_number".to_string(),
        data.exchange_reference_number.clone(),
    );
    fields.insert(
        "exchange_transaction_id".to_string(),
        data.exchange_transaction_id.clone(),
    );
    fields.insert("order_number".to_string(), data.order_number.clone());
    fields.insert("payer_bank_name".to_string(), data.payer_bank_name.clone());
    fields.insert("payer_email".to_string(), data.payer_email.clone());
    fields.insert("payer_name".to_string(), data.payer_name.clone());
    fields.insert("record_type".to_string(), data.record_type.clone());
    fields.insert("status".to_string(), data.status.clone());
    fields.insert(
        "status_description".to_string(),
        data.status_description.clone(),
    );
    fields.insert("transaction_id".to_string(), data.transaction_id.clone());
    create_checksum_value(secret_key, &fields) == data.checksum
}

pub fn verify_pre_transaction(data: &PreTransactionCallbackData, secret_key: &str) -> bool {
    let mut fields = BTreeMap::new();
    fields.insert(
        "exchange_reference_number".to_string(),
        data.exchange_reference_number.clone(),
    );
    fields.insert("order_number".to_string(), data.order_number.clone());
    fields.insert("record_type".to_string(), data.record_type.clone());
    create_checksum_value(secret_key, &fields) == data.checksum
}

pub fn verify_return_url(data: &ReturnUrlCallbackData, secret_key: &str) -> bool {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), data.amount.clone());
    fields.insert("currency".to_string(), data.currency.clone());
    fields.insert(
        "exchange_reference_number".to_string(),
        data.exchange_reference_number.clone(),
    );
    fields.insert(
        "exchange_transaction_id".to_string(),
        data.exchange_transaction_id.clone(),
    );
    fields.insert("order_number".to_string(), data.order_number.clone());
    fields.insert("payer_bank_name".to_string(), data.payer_bank_name.clone());
    fields.insert("status".to_string(), data.status.clone());
    fields.insert(
        "status_description".to_string(),
        data.status_description.clone(),
    );
    fields.insert("transaction_id".to_string(), data.transaction_id.clone());
    create_checksum_value(secret_key, &fields) == data.checksum
}

pub fn verify_direct_debit_bank_approval(
    data: &DirectDebitBankApprovalCallbackData,
    secret_key: &str,
) -> bool {
    let mut fields = BTreeMap::new();
    fields.insert(
        "application_type".to_string(),
        data.application_type.clone(),
    );
    fields.insert("approval_date".to_string(), data.approval_date.clone());
    fields.insert("approval_status".to_string(), data.approval_status.clone());
    fields.insert("mandate_id".to_string(), data.mandate_id.clone());
    fields.insert(
        "mandate_reference_number".to_string(),
        data.mandate_reference_number.clone(),
    );
    fields.insert("order_number".to_string(), data.order_number.clone());
    fields.insert(
        "payer_bank_account_no".to_string(),
        data.payer_bank_account_no.clone(),
    );
    fields.insert("payer_bank_code".to_string(), data.payer_bank_code.clone());
    fields.insert(
        "payer_bank_code_hashed".to_string(),
        data.payer_bank_code_hashed.clone(),
    );
    fields.insert("record_type".to_string(), data.record_type.clone());
    create_checksum_value(secret_key, &fields) == data.checksum
}

pub fn verify_direct_debit_authorization(
    data: &DirectDebitAuthorizationCallbackData,
    secret_key: &str,
) -> bool {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), data.amount.clone());
    fields.insert("currency".to_string(), data.currency.clone());
    fields.insert("datetime".to_string(), data.datetime.clone());
    fields.insert(
        "exchange_reference_number".to_string(),
        data.exchange_reference_number.clone(),
    );
    fields.insert(
        "exchange_transaction_id".to_string(),
        data.exchange_transaction_id.clone(),
    );
    fields.insert("mandate_id".to_string(), data.mandate_id.clone());
    fields.insert("order_number".to_string(), data.order_number.clone());
    fields.insert("payer_bank_name".to_string(), data.payer_bank_name.clone());
    fields.insert("payer_email".to_string(), data.payer_email.clone());
    fields.insert("payer_name".to_string(), data.payer_name.clone());
    fields.insert("record_type".to_string(), data.record_type.clone());
    fields.insert("status".to_string(), data.status.clone());
    fields.insert(
        "status_description".to_string(),
        data.status_description.clone(),
    );
    fields.insert("transaction_id".to_string(), data.transaction_id.clone());
    create_checksum_value(secret_key, &fields) == data.checksum
}

pub fn verify_direct_debit_transaction(
    data: &DirectDebitTransactionCallbackData,
    secret_key: &str,
) -> bool {
    let mut fields = BTreeMap::new();
    fields.insert("amount".to_string(), data.amount.clone());
    fields.insert("batch_number".to_string(), data.batch_number.clone());
    fields.insert("cycle".to_string(), data.cycle.clone());
    fields.insert("datetime".to_string(), data.datetime.clone());
    fields.insert("mandate_id".to_string(), data.mandate_id.clone());
    fields.insert(
        "mandate_reference_number".to_string(),
        data.mandate_reference_number.clone(),
    );
    fields.insert("record_type".to_string(), data.record_type.clone());
    fields.insert(
        "reference_number".to_string(),
        data.reference_number.clone(),
    );
    fields.insert("status".to_string(), data.status.clone());
    fields.insert(
        "status_description".to_string(),
        data.status_description.clone(),
    );
    fields.insert("transaction_id".to_string(), data.transaction_id.clone());
    create_checksum_value(secret_key, &fields) == data.checksum
}
