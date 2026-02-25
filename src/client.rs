use crate::config::BayarcashConfig;
use crate::error::{BayarcashError, Result};
use crate::http::HttpClient;
use crate::types::*;
use std::time::Duration;

/// Bayarcash payment gateway client
pub struct Bayarcash {
    pub(crate) config: BayarcashConfig,
    pub(crate) http: HttpClient,
}

/// Builder for constructing a Bayarcash client
pub struct BayarcashBuilder {
    token: String,
    sandbox: bool,
    api_version: ApiVersion,
    timeout: Duration,
}

impl Bayarcash {
    /// Create a new client with defaults (production, v2, 30s timeout)
    pub fn new(token: &str) -> Result<Self> {
        Self::builder(token).build()
    }

    /// Create a builder for more configuration options
    pub fn builder(token: &str) -> BayarcashBuilder {
        BayarcashBuilder {
            token: token.to_string(),
            sandbox: false,
            api_version: ApiVersion::V2,
            timeout: Duration::from_secs(30),
        }
    }

    /// Helper to enforce v3-only methods
    fn require_v3(&self, method_name: &str) -> Result<()> {
        if self.config.api_version != ApiVersion::V3 {
            return Err(BayarcashError::ApiVersionMismatch(method_name.to_string()));
        }
        Ok(())
    }

    // ===== Payment Intents =====

    pub async fn create_payment_intent(
        &self,
        request: &PaymentIntentRequest,
    ) -> Result<PaymentIntent> {
        self.http.post("payment-intents", request).await
    }

    pub async fn get_payment_intent(&self, id: &str) -> Result<PaymentIntent> {
        self.require_v3("get_payment_intent")?;
        self.http.get(&format!("payment-intents/{}", id)).await
    }

    // ===== FPX Banks =====

    pub async fn fpx_banks_list(&self) -> Result<Vec<FpxBank>> {
        self.http.get("banks").await
    }

    // ===== Portals =====

    pub async fn get_portals(&self) -> Result<Vec<Portal>> {
        let raw: serde_json::Value = self.http.get("portals").await?;
        if raw.is_array() {
            return serde_json::from_value(raw).map_err(|e| BayarcashError::Other(e.to_string()));
        }
        if let Some(data) = raw.get("data") {
            return serde_json::from_value(data.clone())
                .map_err(|e| BayarcashError::Other(e.to_string()));
        }
        Ok(vec![])
    }

    pub async fn get_channels(&self, portal_key: &str) -> Result<Vec<PaymentChannelInfo>> {
        let portals = self.get_portals().await?;
        Ok(portals
            .into_iter()
            .find(|p| p.portal_key == portal_key)
            .map(|p| p.payment_channels)
            .unwrap_or_default())
    }

    // ===== Transactions =====

    pub async fn get_transaction(&self, id: &str) -> Result<Transaction> {
        self.http.get(&format!("transactions/{}", id)).await
    }

    pub async fn get_all_transactions(
        &self,
        params: &TransactionQueryParams,
    ) -> Result<PaginatedResponse<Transaction>> {
        self.require_v3("get_all_transactions")?;
        let query = serde_urlencoded::to_string(params).unwrap_or_default();
        let endpoint = if query.is_empty() {
            "transactions".to_string()
        } else {
            format!("transactions?{}", query)
        };
        self.http.get(&endpoint).await
    }

    pub async fn get_transactions_by_order(&self, order_number: &str) -> Result<Vec<Transaction>> {
        self.require_v3("get_transactions_by_order")?;
        let resp: PaginatedResponse<Transaction> = self
            .http
            .get(&format!(
                "transactions?order_number={}",
                urlencoding::encode(order_number)
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transactions_by_email(&self, email: &str) -> Result<Vec<Transaction>> {
        self.require_v3("get_transactions_by_email")?;
        let resp: PaginatedResponse<Transaction> = self
            .http
            .get(&format!(
                "transactions?payer_email={}",
                urlencoding::encode(email)
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transactions_by_status(&self, status: &str) -> Result<Vec<Transaction>> {
        self.require_v3("get_transactions_by_status")?;
        let resp: PaginatedResponse<Transaction> = self
            .http
            .get(&format!(
                "transactions?status={}",
                urlencoding::encode(status)
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transactions_by_channel(
        &self,
        channel: PaymentChannel,
    ) -> Result<Vec<Transaction>> {
        self.require_v3("get_transactions_by_channel")?;
        let resp: PaginatedResponse<Transaction> = self
            .http
            .get(&format!("transactions?payment_channel={}", channel as u8))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transaction_by_reference(
        &self,
        reference: &str,
    ) -> Result<Option<Transaction>> {
        self.require_v3("get_transaction_by_reference")?;
        let resp: PaginatedResponse<Transaction> = self
            .http
            .get(&format!(
                "transactions?exchange_reference_number={}",
                urlencoding::encode(reference)
            ))
            .await?;
        Ok(resp.data.into_iter().next())
    }

    // ===== FPX Direct Debit =====

    pub async fn create_fpx_direct_debit_enrollment(
        &self,
        data: &impl serde::Serialize,
    ) -> Result<FpxDirectDebitApplication> {
        self.http.post("mandates", data).await
    }

    pub async fn create_fpx_direct_debit_maintenance(
        &self,
        mandate_id: &str,
        data: &impl serde::Serialize,
    ) -> Result<FpxDirectDebitApplication> {
        self.http
            .put(&format!("mandates/{}", mandate_id), data)
            .await
    }

    pub async fn create_fpx_direct_debit_termination(
        &self,
        mandate_id: &str,
        data: &impl serde::Serialize,
    ) -> Result<FpxDirectDebitApplication> {
        self.http
            .delete(&format!("mandates/{}", mandate_id), data)
            .await
    }

    pub async fn get_fpx_direct_debit_transaction(&self, id: &str) -> Result<Transaction> {
        self.http
            .get(&format!("mandates/transactions/{}", id))
            .await
    }

    pub async fn get_fpx_direct_debit(&self, id: &str) -> Result<FpxDirectDebit> {
        self.http.get(&format!("mandates/{}", id)).await
    }

    // ===== Manual Bank Transfer =====

    pub async fn create_manual_bank_transfer(
        &self,
        data: &ManualBankTransferRequest,
        _allow_redirect: bool,
    ) -> Result<serde_json::Value> {
        if data.payment_gateway != 2 {
            return Err(BayarcashError::Other(
                "payment_gateway must be 2 for manual transfers".to_string(),
            ));
        }
        let url = format!(
            "{}/manual-bank-transfer",
            self.config.manual_transfer_base_url()
        );

        let mut form = reqwest::multipart::Form::new()
            .text("portal_key", data.portal_key.clone())
            .text("buyer_name", data.buyer_name.clone())
            .text("buyer_email", data.buyer_email.clone())
            .text("order_amount", data.order_amount.to_string())
            .text("order_no", data.order_no.clone())
            .text("payment_gateway", data.payment_gateway.to_string())
            .text("merchant_bank_name", data.merchant_bank_name.clone())
            .text("merchant_bank_account", data.merchant_bank_account.clone())
            .text(
                "merchant_bank_account_holder",
                data.merchant_bank_account_holder.clone(),
            )
            .text("bank_transfer_type", data.bank_transfer_type.clone())
            .text("bank_transfer_notes", data.bank_transfer_notes.clone());

        if let Some(ref date) = data.bank_transfer_date {
            form = form.text("bank_transfer_date", date.clone());
        }
        if let Some(ref path) = data.proof_of_payment_path {
            let file_bytes = tokio::fs::read(path)
                .await
                .map_err(|e| BayarcashError::Other(format!("Failed to read file: {}", e)))?;
            let filename = std::path::Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "proof.jpg".to_string());
            form = form.part(
                "proof_of_payment",
                reqwest::multipart::Part::bytes(file_bytes).file_name(filename),
            );
        }

        self.http.post_multipart(&url, form).await
    }

    pub async fn update_manual_bank_transfer_status(
        &self,
        ref_no: &str,
        status: &str,
        amount: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/manual-bank-transfer/update-status",
            self.config.manual_transfer_base_url()
        );
        #[derive(serde::Serialize)]
        struct UpdateBody {
            ref_no: String,
            status: String,
            amount: String,
        }
        let body = UpdateBody {
            ref_no: ref_no.to_string(),
            status: status.to_string(),
            amount: amount.to_string(),
        };
        self.http.post_absolute(&url, &body).await
    }
}

impl BayarcashBuilder {
    pub fn sandbox(mut self, enabled: bool) -> Self {
        self.sandbox = enabled;
        self
    }
    pub fn api_version(mut self, version: ApiVersion) -> Self {
        self.api_version = version;
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<Bayarcash> {
        let config = BayarcashConfig {
            token: self.token.clone(),
            sandbox: self.sandbox,
            api_version: self.api_version,
            timeout: self.timeout,
        };
        let http = HttpClient::new(config.base_url(), &self.token, self.timeout)?;
        Ok(Bayarcash { config, http })
    }

    /// Build with a custom base URL (for testing with mockito)
    pub fn build_with_base_url(self, base_url: &str) -> Result<Bayarcash> {
        let config = BayarcashConfig {
            token: self.token.clone(),
            sandbox: self.sandbox,
            api_version: self.api_version,
            timeout: self.timeout,
        };
        let http = HttpClient::new(base_url, &self.token, self.timeout)?;
        Ok(Bayarcash { config, http })
    }
}
