use bayarcash_sdk::{AppConfig, PaymentIntentRequest, TransactionQueryParams};
use rmcp::{
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, ServerHandler, ServiceExt,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone)]
struct BayarcashMcp {
    #[allow(dead_code)]
    config: AppConfig,
}

impl BayarcashMcp {
    fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

// === Tool parameter structs ===
// These structs are consumed by the #[tool(aggr)] macro; allow dead_code for all of them.
#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct CreatePaymentParams {
    /// Payment channel (1=FPX, 2=Manual, 3=DD, 5=DuitNow DOBW, 6=DuitNow QR, 12=Credit Card)
    channel: u8,
    /// Order number
    order_number: String,
    /// Payment amount
    amount: f64,
    /// Payer full name
    payer_name: String,
    /// Payer email address
    payer_email: String,
    /// Payer phone number (optional)
    payer_phone: Option<String>,
    /// Currency code (default: MYR)
    currency: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct IdParam {
    /// Resource ID
    id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct ListTransactionsParams {
    /// Filter by order number
    order_number: Option<String>,
    /// Filter by status
    status: Option<String>,
    /// Filter by payment channel number
    payment_channel: Option<u8>,
    /// Filter by payer email
    payer_email: Option<String>,
    /// Filter by exchange reference number
    exchange_reference_number: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct PortalKeyParam {
    /// Portal key
    portal_key: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct ChecksumPaymentParams {
    /// Secret key for HMAC
    secret_key: String,
    /// Payment channel number
    channel: u8,
    /// Order number
    order_number: String,
    /// Amount
    amount: f64,
    /// Payer name
    payer_name: String,
    /// Payer email
    payer_email: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct VerifyCallbackParams {
    /// Secret key for verification
    secret_key: String,
    /// Callback type: transaction, pre_transaction, return_url, dd_approval, dd_authorization, dd_transaction
    callback_type: String,
    /// Raw callback JSON data
    callback_data: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct MandateCreateParams {
    /// Enrollment data as JSON object
    data: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, JsonSchema)]
struct MandateUpdateParams {
    /// Mandate ID
    mandate_id: String,
    /// Update data as JSON object
    data: serde_json::Value,
}

#[tool(tool_box)]
impl BayarcashMcp {
    /// Create a payment intent
    #[tool(
        description = "Create a Bayarcash payment intent. Returns the payment URL and intent details."
    )]
    async fn create_payment(&self, #[tool(aggr)] params: CreatePaymentParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        let request = PaymentIntentRequest {
            payment_channel: params.channel,
            order_number: params.order_number,
            amount: params.amount,
            payer_name: params.payer_name,
            payer_email: params.payer_email,
            payer_telephone_number: params.payer_phone,
            currency: params.currency,
            callback_url: None,
            return_url: None,
            metadata: None,
            checksum: None,
        };

        match client.create_payment_intent(&request).await {
            Ok(intent) => serde_json::to_string_pretty(&intent)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Get a payment intent by ID (v3 only)
    #[tool(description = "Get a payment intent by ID. Requires v3 API.")]
    async fn get_payment(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.get_payment_intent(&params.id).await {
            Ok(intent) => serde_json::to_string_pretty(&intent)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Get a transaction by ID
    #[tool(description = "Get a transaction by its ID.")]
    async fn get_transaction(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.get_transaction(&params.id).await {
            Ok(tx) => serde_json::to_string_pretty(&tx)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// List transactions with optional filters
    #[tool(
        description = "List transactions with optional filters (order_number, status, payment_channel, payer_email, exchange_reference_number). Requires v3 API."
    )]
    async fn list_transactions(&self, #[tool(aggr)] params: ListTransactionsParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        let query = TransactionQueryParams {
            order_number: params.order_number,
            status: params.status,
            payment_channel: params.payment_channel,
            payer_email: params.payer_email,
            exchange_reference_number: params.exchange_reference_number,
        };

        match client.get_all_transactions(&query).await {
            Ok(resp) => serde_json::to_string_pretty(&resp)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// List FPX banks
    #[tool(description = "List all available FPX banks.")]
    async fn list_banks(&self) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.fpx_banks_list().await {
            Ok(banks) => serde_json::to_string_pretty(&banks)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// List portals
    #[tool(description = "List all portals associated with this account.")]
    async fn list_portals(&self) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.get_portals().await {
            Ok(portals) => serde_json::to_string_pretty(&portals)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Get payment channels for a portal
    #[tool(description = "Get available payment channels for a specific portal.")]
    async fn get_channels(&self, #[tool(aggr)] params: PortalKeyParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.get_channels(&params.portal_key).await {
            Ok(channels) => serde_json::to_string_pretty(&channels)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Generate payment intent checksum
    #[tool(description = "Generate an HMAC-SHA256 checksum for a payment intent request.")]
    async fn generate_checksum(&self, #[tool(aggr)] params: ChecksumPaymentParams) -> String {
        let checksum = bayarcash_sdk::checksum::payment_intent(
            &params.secret_key,
            params.channel,
            &params.order_number,
            params.amount,
            &params.payer_name,
            &params.payer_email,
        );
        format!("{{\"checksum\":\"{}\"}}", checksum)
    }

    /// Verify callback signature
    #[tool(
        description = "Verify a Bayarcash callback signature. callback_type must be one of: transaction, pre_transaction, return_url, dd_approval, dd_authorization, dd_transaction"
    )]
    async fn verify_callback(&self, #[tool(aggr)] params: VerifyCallbackParams) -> String {
        let result = match params.callback_type.as_str() {
            "transaction" => {
                match serde_json::from_value::<bayarcash_sdk::TransactionCallbackData>(
                    params.callback_data,
                ) {
                    Ok(data) => bayarcash_sdk::verification::verify_transaction(&data, &params.secret_key),
                    Err(e) => return format!("Failed to parse callback data: {}", e),
                }
            }
            "pre_transaction" => {
                match serde_json::from_value::<bayarcash_sdk::PreTransactionCallbackData>(
                    params.callback_data,
                ) {
                    Ok(data) => {
                        bayarcash_sdk::verification::verify_pre_transaction(&data, &params.secret_key)
                    }
                    Err(e) => return format!("Failed to parse callback data: {}", e),
                }
            }
            "return_url" => {
                match serde_json::from_value::<bayarcash_sdk::ReturnUrlCallbackData>(
                    params.callback_data,
                ) {
                    Ok(data) => {
                        bayarcash_sdk::verification::verify_return_url(&data, &params.secret_key)
                    }
                    Err(e) => return format!("Failed to parse callback data: {}", e),
                }
            }
            "dd_approval" => {
                match serde_json::from_value::<bayarcash_sdk::DirectDebitBankApprovalCallbackData>(
                    params.callback_data,
                ) {
                    Ok(data) => bayarcash_sdk::verification::verify_direct_debit_bank_approval(
                        &data,
                        &params.secret_key,
                    ),
                    Err(e) => return format!("Failed to parse callback data: {}", e),
                }
            }
            "dd_authorization" => {
                match serde_json::from_value::<bayarcash_sdk::DirectDebitAuthorizationCallbackData>(
                    params.callback_data,
                ) {
                    Ok(data) => bayarcash_sdk::verification::verify_direct_debit_authorization(
                        &data,
                        &params.secret_key,
                    ),
                    Err(e) => return format!("Failed to parse callback data: {}", e),
                }
            }
            "dd_transaction" => {
                match serde_json::from_value::<bayarcash_sdk::DirectDebitTransactionCallbackData>(
                    params.callback_data,
                ) {
                    Ok(data) => bayarcash_sdk::verification::verify_direct_debit_transaction(
                        &data,
                        &params.secret_key,
                    ),
                    Err(e) => return format!("Failed to parse callback data: {}", e),
                }
            }
            other => {
                return format!(
                    "Unknown callback type '{}'. Use: transaction, pre_transaction, return_url, dd_approval, dd_authorization, dd_transaction",
                    other
                )
            }
        };
        format!("{{\"valid\":{}}}", result)
    }

    /// Create an FPX Direct Debit enrollment (mandate)
    #[tool(
        description = "Create an FPX Direct Debit enrollment mandate. Pass enrollment data as a JSON object."
    )]
    async fn create_mandate(&self, #[tool(aggr)] params: MandateCreateParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client
            .create_fpx_direct_debit_enrollment(&params.data)
            .await
        {
            Ok(app) => serde_json::to_string_pretty(&app)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Update an FPX Direct Debit mandate
    #[tool(description = "Update (maintenance) an FPX Direct Debit mandate by mandate ID.")]
    async fn update_mandate(&self, #[tool(aggr)] params: MandateUpdateParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client
            .create_fpx_direct_debit_maintenance(&params.mandate_id, &params.data)
            .await
        {
            Ok(app) => serde_json::to_string_pretty(&app)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Get a direct debit mandate by ID
    #[tool(description = "Get an FPX Direct Debit mandate by its ID.")]
    async fn get_mandate(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.get_fpx_direct_debit(&params.id).await {
            Ok(dd) => serde_json::to_string_pretty(&dd)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Get a direct debit transaction by ID
    #[tool(description = "Get an FPX Direct Debit transaction by its ID.")]
    async fn get_mandate_transaction(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("Error building client: {}", e),
        };

        match client.get_fpx_direct_debit_transaction(&params.id).await {
            Ok(tx) => serde_json::to_string_pretty(&tx)
                .unwrap_or_else(|e| format!("Serialization error: {}", e)),
            Err(e) => format!("Error: {}", e),
        }
    }
}

impl ServerHandler for BayarcashMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "bayarcash-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "Bayarcash MCP server for managing payments, transactions, portals, and FPX Direct Debit mandates. \
                Set BAYARCASH_TOKEN env var (and optionally BAYARCASH_SECRET_KEY, BAYARCASH_SANDBOX=true, BAYARCASH_API_VERSION=v3)."
                    .to_string(),
            ),
            protocol_version: Default::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load();
    let server = BayarcashMcp::new(config);

    let transport = rmcp::transport::io::stdio();
    let service = server.serve(transport).await?;
    service.waiting().await?;

    Ok(())
}
