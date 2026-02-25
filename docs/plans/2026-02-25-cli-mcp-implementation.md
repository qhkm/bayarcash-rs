# Bayarcash CLI + MCP Server Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a `bayarcash` CLI binary and `bayarcash-mcp` MCP server binary to the existing SDK crate, enabling AI agents to interact with the Bayarcash payment gateway via command line or MCP tools.

**Architecture:** Both binaries live in `src/bin/` and share the existing library code. The CLI uses clap with subcommands mirroring the SDK surface. The MCP server uses rmcp with `#[tool]` macros exposing the same operations. A shared config module handles credential loading from config file, env vars, and CLI flags.

**Tech Stack:** clap 4 (derive), rmcp (server + macros), dirs, toml, serde, tokio

---

### Task 1: Add Dependencies and Binary Targets

**Files:**
- Modify: `~/rust/bayarcash-sdk/Cargo.toml`

**Step 1: Add new dependencies and [[bin]] targets to Cargo.toml**

Add to `[dependencies]`:
```toml
clap = { version = "4", features = ["derive"] }
rmcp = { version = "0.1", features = ["server", "macros", "transport-io"] }
dirs = "6"
toml = "0.8"
schemars = "0.8"
```

Add binary targets:
```toml
[[bin]]
name = "bayarcash"
path = "src/bin/cli.rs"

[[bin]]
name = "bayarcash-mcp"
path = "src/bin/mcp.rs"
```

**Step 2: Create placeholder bin files so cargo check passes**

Create `src/bin/cli.rs`:
```rust
fn main() {
    println!("bayarcash CLI - coming soon");
}
```

Create `src/bin/mcp.rs`:
```rust
fn main() {
    println!("bayarcash MCP server - coming soon");
}
```

**Step 3: Verify it compiles**

Run: `cd ~/rust/bayarcash-sdk && cargo check`
Expected: SUCCESS

**Step 4: Commit**

```bash
git add Cargo.toml src/bin/
git commit -m "chore: add CLI and MCP binary targets with dependencies"
```

---

### Task 2: Config Module

**Files:**
- Create: `~/rust/bayarcash-sdk/src/app_config.rs`
- Modify: `~/rust/bayarcash-sdk/src/lib.rs`

**Step 1: Create src/app_config.rs**

This module handles loading config from file, env vars, and provides defaults. It is public so both binaries can use it.

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application config loaded from file + env vars
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    #[serde(default)]
    pub sandbox: Option<bool>,
    #[serde(default)]
    pub api_version: Option<String>,
}

impl AppConfig {
    /// Load config from ~/.bayarcash/config.toml, then overlay env vars.
    /// CLI flags are applied by the caller after this.
    pub fn load() -> Self {
        let mut config = Self::from_file().unwrap_or_default();
        config.overlay_env();
        config
    }

    /// Read from config file if it exists
    fn from_file() -> Option<Self> {
        let path = Self::config_path()?;
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    /// Overlay environment variables (higher priority than file)
    fn overlay_env(&mut self) {
        if let Ok(v) = std::env::var("BAYARCASH_TOKEN") {
            self.token = Some(v);
        }
        if let Ok(v) = std::env::var("BAYARCASH_SECRET_KEY") {
            self.secret_key = Some(v);
        }
        if let Ok(v) = std::env::var("BAYARCASH_SANDBOX") {
            self.sandbox = Some(v == "true" || v == "1");
        }
        if let Ok(v) = std::env::var("BAYARCASH_API_VERSION") {
            self.api_version = Some(v);
        }
    }

    /// Get the config file path: ~/.bayarcash/config.toml
    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".bayarcash").join("config.toml"))
    }

    /// Get token or error
    pub fn require_token(&self) -> Result<&str, String> {
        self.token.as_deref().ok_or_else(|| {
            "No API token. Set BAYARCASH_TOKEN env var or add to ~/.bayarcash/config.toml".to_string()
        })
    }

    /// Get secret key or error
    pub fn require_secret_key(&self) -> Result<&str, String> {
        self.secret_key.as_deref().ok_or_else(|| {
            "No secret key. Set BAYARCASH_SECRET_KEY env var or add to ~/.bayarcash/config.toml".to_string()
        })
    }

    /// Resolve sandbox flag (default: false)
    pub fn is_sandbox(&self) -> bool {
        self.sandbox.unwrap_or(false)
    }

    /// Resolve API version (default: "v2")
    pub fn resolved_api_version(&self) -> &str {
        self.api_version.as_deref().unwrap_or("v2")
    }

    /// Build a Bayarcash client from this config
    pub fn build_client(&self) -> Result<crate::Bayarcash, crate::BayarcashError> {
        let token = self.require_token().map_err(crate::BayarcashError::Other)?;
        let api_version = match self.resolved_api_version() {
            "v3" => crate::ApiVersion::V3,
            _ => crate::ApiVersion::V2,
        };
        crate::Bayarcash::builder(token)
            .sandbox(self.is_sandbox())
            .api_version(api_version)
            .build()
    }
}
```

**Step 2: Add to lib.rs**

Add `pub mod app_config;` and re-export: `pub use app_config::AppConfig;`

**Step 3: Verify it compiles**

Run: `cargo check`
Expected: SUCCESS

**Step 4: Commit**

```bash
git add src/app_config.rs src/lib.rs
git commit -m "feat: add AppConfig for file + env var credential loading"
```

---

### Task 3: CLI - Argument Parsing with clap

**Files:**
- Create: `~/rust/bayarcash-sdk/src/bin/cli.rs`

**Step 1: Write the full CLI with clap derive**

```rust
use bayarcash_sdk::{
    AppConfig, Bayarcash, PaymentChannel, PaymentIntentRequest, TransactionQueryParams,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bayarcash", version, about = "Bayarcash Payment Gateway CLI")]
struct Cli {
    /// Use sandbox environment
    #[arg(long, global = true)]
    sandbox: bool,

    /// API version (v2 or v3)
    #[arg(long, global = true, default_value = "v2")]
    api_version: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize config file
    Init,

    /// Payment intent operations
    Payment {
        #[command(subcommand)]
        command: PaymentCommands,
    },

    /// Transaction operations
    Transaction {
        #[command(subcommand)]
        command: TransactionCommands,
    },

    /// FPX bank operations
    Banks {
        #[command(subcommand)]
        command: BanksCommands,
    },

    /// Portal operations
    Portal {
        #[command(subcommand)]
        command: PortalCommands,
    },

    /// Checksum generation
    Checksum {
        #[command(subcommand)]
        command: ChecksumCommands,
    },

    /// Callback verification
    Verify {
        #[command(subcommand)]
        command: VerifyCommands,
    },

    /// FPX Direct Debit mandate operations
    Mandate {
        #[command(subcommand)]
        command: MandateCommands,
    },
}

#[derive(Subcommand)]
enum PaymentCommands {
    /// Create a payment intent
    Create {
        #[arg(long)]
        channel: u8,
        #[arg(long)]
        order: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long, default_value = "MYR")]
        currency: String,
    },
    /// Get a payment intent by ID (v3 only)
    Get {
        /// Payment intent ID
        id: String,
    },
}

#[derive(Subcommand)]
enum TransactionCommands {
    /// Get a transaction by ID
    Get {
        /// Transaction ID
        id: String,
    },
    /// List transactions with filters (v3 only)
    List {
        #[arg(long)]
        order: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        channel: Option<u8>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        reference: Option<String>,
    },
}

#[derive(Subcommand)]
enum BanksCommands {
    /// List FPX banks
    List,
}

#[derive(Subcommand)]
enum PortalCommands {
    /// List portals
    List,
    /// List payment channels for a portal
    Channels {
        /// Portal key
        key: String,
    },
}

#[derive(Subcommand)]
enum ChecksumCommands {
    /// Generate payment intent checksum
    Payment {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        channel: u8,
        #[arg(long)]
        order: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
    },
    /// Generate DD enrollment checksum
    Enrollment {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        order: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        phone: String,
        #[arg(long)]
        id_type: String,
        #[arg(long)]
        id_value: String,
        #[arg(long)]
        reason: String,
        #[arg(long)]
        frequency: String,
    },
    /// Generate DD maintenance checksum
    Maintenance {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        email: String,
        #[arg(long)]
        phone: String,
        #[arg(long)]
        reason: String,
        #[arg(long)]
        frequency: String,
    },
}

#[derive(Subcommand)]
enum VerifyCommands {
    /// Verify transaction callback (pass JSON as argument or pipe via stdin)
    Transaction {
        /// JSON callback data (or use stdin)
        json: Option<String>,
        #[arg(long)]
        secret: String,
    },
    /// Verify pre-transaction callback
    PreTransaction {
        json: Option<String>,
        #[arg(long)]
        secret: String,
    },
    /// Verify return URL callback
    ReturnUrl {
        json: Option<String>,
        #[arg(long)]
        secret: String,
    },
    /// Verify DD bank approval callback
    DdApproval {
        json: Option<String>,
        #[arg(long)]
        secret: String,
    },
    /// Verify DD authorization callback
    DdAuthorization {
        json: Option<String>,
        #[arg(long)]
        secret: String,
    },
    /// Verify DD transaction callback
    DdTransaction {
        json: Option<String>,
        #[arg(long)]
        secret: String,
    },
}

#[derive(Subcommand)]
enum MandateCommands {
    /// Create DD enrollment (pass JSON body)
    Create {
        /// JSON body
        json: String,
    },
    /// Update DD maintenance
    Update {
        /// Mandate ID
        id: String,
        /// JSON body
        json: String,
    },
    /// Terminate DD mandate
    Terminate {
        /// Mandate ID
        id: String,
        /// JSON body
        json: String,
    },
    /// Get mandate details
    Get {
        /// Mandate ID
        id: String,
    },
    /// Get mandate transaction
    Transaction {
        /// Transaction ID
        id: String,
    },
}

fn read_json_input(arg: Option<String>) -> Result<String, String> {
    if let Some(json) = arg {
        Ok(json)
    } else {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).map_err(|e| e.to_string())?;
        Ok(buf)
    }
}

fn print_json<T: serde::Serialize>(value: &T) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}

fn print_error(msg: &str) {
    eprintln!("{}", serde_json::json!({"error": msg}));
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut config = AppConfig::load();

    // CLI flags override
    if cli.sandbox {
        config.sandbox = Some(true);
    }
    if cli.api_version != "v2" {
        config.api_version = Some(cli.api_version.clone());
    }

    let result = run(cli.command, config).await;
    if let Err(e) = result {
        print_error(&e);
        std::process::exit(1);
    }
}

async fn run(command: Commands, config: AppConfig) -> Result<(), String> {
    match command {
        Commands::Init => {
            let path = AppConfig::config_path().ok_or("Cannot determine home directory")?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let template = r#"token = "your_api_token_here"
secret_key = "your_secret_key_here"
sandbox = false
api_version = "v2"
"#;
            std::fs::write(&path, template).map_err(|e| e.to_string())?;
            println!("{}", serde_json::json!({"status": "ok", "path": path.display().to_string()}));
            Ok(())
        }

        Commands::Payment { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                PaymentCommands::Create { channel, order, amount, name, email, phone, currency } => {
                    let req = PaymentIntentRequest {
                        payment_channel: channel,
                        order_number: order,
                        amount,
                        payer_name: name,
                        payer_email: email,
                        payer_telephone_number: phone,
                        currency: Some(currency),
                        callback_url: None,
                        return_url: None,
                        metadata: None,
                        checksum: None,
                    };
                    let result = client.create_payment_intent(&req).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                PaymentCommands::Get { id } => {
                    let result = client.get_payment_intent(&id).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }

        Commands::Transaction { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                TransactionCommands::Get { id } => {
                    let result = client.get_transaction(&id).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                TransactionCommands::List { order, status, channel, email, reference } => {
                    let params = TransactionQueryParams {
                        order_number: order,
                        status,
                        payment_channel: channel,
                        payer_email: email,
                        exchange_reference_number: reference,
                    };
                    let result = client.get_all_transactions(&params).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }

        Commands::Banks { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                BanksCommands::List => {
                    let result = client.fpx_banks_list().await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }

        Commands::Portal { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                PortalCommands::List => {
                    let result = client.get_portals().await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                PortalCommands::Channels { key } => {
                    let result = client.get_channels(&key).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }

        Commands::Checksum { command } => {
            match command {
                ChecksumCommands::Payment { secret, channel, order, amount, name, email } => {
                    let cs = bayarcash_sdk::checksum::payment_intent(&secret, channel, &order, amount, &name, &email);
                    println!("{}", serde_json::json!({"checksum": cs}));
                    Ok(())
                }
                ChecksumCommands::Enrollment { secret, order, amount, name, email, phone, id_type, id_value, reason, frequency } => {
                    let cs = bayarcash_sdk::checksum::fpx_direct_debit_enrollment(&secret, &order, amount, &name, &email, &phone, &id_type, &id_value, &reason, &frequency);
                    println!("{}", serde_json::json!({"checksum": cs}));
                    Ok(())
                }
                ChecksumCommands::Maintenance { secret, amount, email, phone, reason, frequency } => {
                    let cs = bayarcash_sdk::checksum::fpx_direct_debit_maintenance(&secret, amount, &email, &phone, &reason, &frequency);
                    println!("{}", serde_json::json!({"checksum": cs}));
                    Ok(())
                }
            }
        }

        Commands::Verify { command } => {
            match command {
                VerifyCommands::Transaction { json, secret } => {
                    let input = read_json_input(json)?;
                    let data: bayarcash_sdk::TransactionCallbackData = serde_json::from_str(&input).map_err(|e| e.to_string())?;
                    let valid = bayarcash_sdk::verification::verify_transaction(&data, &secret);
                    println!("{}", serde_json::json!({"valid": valid}));
                    Ok(())
                }
                VerifyCommands::PreTransaction { json, secret } => {
                    let input = read_json_input(json)?;
                    let data: bayarcash_sdk::PreTransactionCallbackData = serde_json::from_str(&input).map_err(|e| e.to_string())?;
                    let valid = bayarcash_sdk::verification::verify_pre_transaction(&data, &secret);
                    println!("{}", serde_json::json!({"valid": valid}));
                    Ok(())
                }
                VerifyCommands::ReturnUrl { json, secret } => {
                    let input = read_json_input(json)?;
                    let data: bayarcash_sdk::ReturnUrlCallbackData = serde_json::from_str(&input).map_err(|e| e.to_string())?;
                    let valid = bayarcash_sdk::verification::verify_return_url(&data, &secret);
                    println!("{}", serde_json::json!({"valid": valid}));
                    Ok(())
                }
                VerifyCommands::DdApproval { json, secret } => {
                    let input = read_json_input(json)?;
                    let data: bayarcash_sdk::DirectDebitBankApprovalCallbackData = serde_json::from_str(&input).map_err(|e| e.to_string())?;
                    let valid = bayarcash_sdk::verification::verify_direct_debit_bank_approval(&data, &secret);
                    println!("{}", serde_json::json!({"valid": valid}));
                    Ok(())
                }
                VerifyCommands::DdAuthorization { json, secret } => {
                    let input = read_json_input(json)?;
                    let data: bayarcash_sdk::DirectDebitAuthorizationCallbackData = serde_json::from_str(&input).map_err(|e| e.to_string())?;
                    let valid = bayarcash_sdk::verification::verify_direct_debit_authorization(&data, &secret);
                    println!("{}", serde_json::json!({"valid": valid}));
                    Ok(())
                }
                VerifyCommands::DdTransaction { json, secret } => {
                    let input = read_json_input(json)?;
                    let data: bayarcash_sdk::DirectDebitTransactionCallbackData = serde_json::from_str(&input).map_err(|e| e.to_string())?;
                    let valid = bayarcash_sdk::verification::verify_direct_debit_transaction(&data, &secret);
                    println!("{}", serde_json::json!({"valid": valid}));
                    Ok(())
                }
            }
        }

        Commands::Mandate { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                MandateCommands::Create { json } => {
                    let data: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    let result = client.create_fpx_direct_debit_enrollment(&data).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Update { id, json } => {
                    let data: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    let result = client.create_fpx_direct_debit_maintenance(&id, &data).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Terminate { id, json } => {
                    let data: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    let result = client.create_fpx_direct_debit_termination(&id, &data).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Get { id } => {
                    let result = client.get_fpx_direct_debit(&id).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Transaction { id } => {
                    let result = client.get_fpx_direct_debit_transaction(&id).await.map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }
    }
}
```

**IMPORTANT:** Some response types (PaymentIntent, Transaction, etc.) need `Serialize` derives to be printed as JSON. You MUST add `#[derive(Serialize)]` to these types in `src/types/`:
- `PaymentIntent` in `types/payment.rs`
- `Transaction` in `types/transaction.rs`
- `PaginatedResponse<T>` and `PaginationMeta` in `types/transaction.rs`
- `FpxBank` in `types/bank.rs`
- `Portal` and `PaymentChannelInfo` in `types/portal.rs`
- `FpxDirectDebitApplication` and `FpxDirectDebit` in `types/direct_debit.rs`

Add `Serialize` to the existing `#[derive(..., Deserialize)]` lines.

**Step 2: Verify it compiles**

Run: `cargo check --bin bayarcash`
Expected: SUCCESS

**Step 3: Test the CLI**

Run: `cargo run --bin bayarcash -- --help`
Expected: Shows usage with all subcommands

Run: `cargo run --bin bayarcash -- checksum payment --secret test123 --channel 1 --order ORD1 --amount 100 --name "John" --email "j@e.com"`
Expected: Prints JSON with checksum

**Step 4: Commit**

```bash
git add src/bin/cli.rs src/types/
git commit -m "feat: add bayarcash CLI with full SDK surface"
```

---

### Task 4: MCP Server

**Files:**
- Create: `~/rust/bayarcash-sdk/src/bin/mcp.rs`

**Step 1: Write the MCP server using rmcp**

The MCP server exposes Bayarcash SDK operations as MCP tools. It uses `#[tool(tool_box)]` macro from rmcp to define tools, and stdio transport.

```rust
use bayarcash_sdk::AppConfig;
use rmcp::model::ServerInfo;
use rmcp::{tool, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone)]
struct BayarcashMcp {
    config: AppConfig,
}

// === Tool parameter structs ===

#[derive(Debug, Deserialize, JsonSchema)]
struct CreatePaymentParams {
    #[schemars(description = "Payment channel (1=FPX, 2=Manual, 3=DD, 5=DuitNow DOBW, 6=DuitNow QR, 12=Credit Card, etc.)")]
    channel: u8,
    #[schemars(description = "Order number")]
    order_number: String,
    #[schemars(description = "Payment amount")]
    amount: f64,
    #[schemars(description = "Payer full name")]
    payer_name: String,
    #[schemars(description = "Payer email address")]
    payer_email: String,
    #[schemars(description = "Payer phone number (optional)")]
    payer_phone: Option<String>,
    #[schemars(description = "Currency code (default: MYR)")]
    currency: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct IdParam {
    #[schemars(description = "Resource ID")]
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListTransactionsParams {
    #[schemars(description = "Filter by order number")]
    order_number: Option<String>,
    #[schemars(description = "Filter by status")]
    status: Option<String>,
    #[schemars(description = "Filter by payment channel number")]
    payment_channel: Option<u8>,
    #[schemars(description = "Filter by payer email")]
    payer_email: Option<String>,
    #[schemars(description = "Filter by exchange reference number")]
    exchange_reference_number: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PortalKeyParam {
    #[schemars(description = "Portal key")]
    portal_key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ChecksumPaymentParams {
    #[schemars(description = "Secret key for HMAC")]
    secret_key: String,
    #[schemars(description = "Payment channel number")]
    channel: u8,
    #[schemars(description = "Order number")]
    order_number: String,
    #[schemars(description = "Amount")]
    amount: f64,
    #[schemars(description = "Payer name")]
    payer_name: String,
    #[schemars(description = "Payer email")]
    payer_email: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct VerifyCallbackParams {
    #[schemars(description = "Secret key for verification")]
    secret_key: String,
    #[schemars(description = "Callback type: transaction, pre_transaction, return_url, dd_approval, dd_authorization, dd_transaction")]
    callback_type: String,
    #[schemars(description = "Raw callback JSON data")]
    callback_data: serde_json::Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MandateCreateParams {
    #[schemars(description = "Enrollment data as JSON object")]
    data: serde_json::Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MandateUpdateParams {
    #[schemars(description = "Mandate ID")]
    mandate_id: String,
    #[schemars(description = "Update data as JSON object")]
    data: serde_json::Value,
}

// === Tool implementations ===

#[tool(tool_box)]
impl BayarcashMcp {
    fn new(config: AppConfig) -> Self {
        Self { config }
    }

    #[tool(description = "Create a payment intent. Returns payment URL for redirect.")]
    async fn create_payment_intent(
        &self,
        #[tool(aggr)] params: CreatePaymentParams,
    ) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        let req = bayarcash_sdk::PaymentIntentRequest {
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
        match client.create_payment_intent(&req).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Get a payment intent by ID (requires API v3)")]
    async fn get_payment_intent(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.get_payment_intent(&params.id).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Get a transaction by ID")]
    async fn get_transaction(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.get_transaction(&params.id).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "List transactions with optional filters (requires API v3)")]
    async fn list_transactions(&self, #[tool(aggr)] params: ListTransactionsParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        let query = bayarcash_sdk::TransactionQueryParams {
            order_number: params.order_number,
            status: params.status,
            payment_channel: params.payment_channel,
            payer_email: params.payer_email,
            exchange_reference_number: params.exchange_reference_number,
        };
        match client.get_all_transactions(&query).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "List all FPX banks")]
    async fn list_banks(&self) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.fpx_banks_list().await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "List all portals")]
    async fn list_portals(&self) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.get_portals().await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Get payment channels for a portal")]
    async fn get_channels(&self, #[tool(aggr)] params: PortalKeyParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.get_channels(&params.portal_key).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Generate HMAC-SHA256 checksum for payment intent")]
    async fn generate_checksum(&self, #[tool(aggr)] params: ChecksumPaymentParams) -> String {
        let cs = bayarcash_sdk::checksum::payment_intent(
            &params.secret_key, params.channel, &params.order_number,
            params.amount, &params.payer_name, &params.payer_email,
        );
        format!("{{\"checksum\": \"{}\"}}", cs)
    }

    #[tool(description = "Verify a callback checksum. Supports types: transaction, pre_transaction, return_url, dd_approval, dd_authorization, dd_transaction")]
    async fn verify_callback(&self, #[tool(aggr)] params: VerifyCallbackParams) -> String {
        let valid = match params.callback_type.as_str() {
            "transaction" => {
                match serde_json::from_value::<bayarcash_sdk::TransactionCallbackData>(params.callback_data) {
                    Ok(data) => bayarcash_sdk::verification::verify_transaction(&data, &params.secret_key),
                    Err(e) => return format!("{{\"error\": \"Invalid data: {}\"}}", e),
                }
            }
            "pre_transaction" => {
                match serde_json::from_value::<bayarcash_sdk::PreTransactionCallbackData>(params.callback_data) {
                    Ok(data) => bayarcash_sdk::verification::verify_pre_transaction(&data, &params.secret_key),
                    Err(e) => return format!("{{\"error\": \"Invalid data: {}\"}}", e),
                }
            }
            "return_url" => {
                match serde_json::from_value::<bayarcash_sdk::ReturnUrlCallbackData>(params.callback_data) {
                    Ok(data) => bayarcash_sdk::verification::verify_return_url(&data, &params.secret_key),
                    Err(e) => return format!("{{\"error\": \"Invalid data: {}\"}}", e),
                }
            }
            "dd_approval" => {
                match serde_json::from_value::<bayarcash_sdk::DirectDebitBankApprovalCallbackData>(params.callback_data) {
                    Ok(data) => bayarcash_sdk::verification::verify_direct_debit_bank_approval(&data, &params.secret_key),
                    Err(e) => return format!("{{\"error\": \"Invalid data: {}\"}}", e),
                }
            }
            "dd_authorization" => {
                match serde_json::from_value::<bayarcash_sdk::DirectDebitAuthorizationCallbackData>(params.callback_data) {
                    Ok(data) => bayarcash_sdk::verification::verify_direct_debit_authorization(&data, &params.secret_key),
                    Err(e) => return format!("{{\"error\": \"Invalid data: {}\"}}", e),
                }
            }
            "dd_transaction" => {
                match serde_json::from_value::<bayarcash_sdk::DirectDebitTransactionCallbackData>(params.callback_data) {
                    Ok(data) => bayarcash_sdk::verification::verify_direct_debit_transaction(&data, &params.secret_key),
                    Err(e) => return format!("{{\"error\": \"Invalid data: {}\"}}", e),
                }
            }
            other => return format!("{{\"error\": \"Unknown callback type: {}\"}}", other),
        };
        format!("{{\"valid\": {}}}", valid)
    }

    #[tool(description = "Create FPX Direct Debit enrollment")]
    async fn create_mandate(&self, #[tool(aggr)] params: MandateCreateParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.create_fpx_direct_debit_enrollment(&params.data).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Update FPX Direct Debit (maintenance)")]
    async fn update_mandate(&self, #[tool(aggr)] params: MandateUpdateParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.create_fpx_direct_debit_maintenance(&params.mandate_id, &params.data).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Terminate FPX Direct Debit mandate")]
    async fn terminate_mandate(&self, #[tool(aggr)] params: MandateUpdateParams) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.create_fpx_direct_debit_termination(&params.mandate_id, &params.data).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Get FPX Direct Debit mandate details")]
    async fn get_mandate(&self, #[tool(aggr)] params: IdParam) -> String {
        let client = match self.config.build_client() {
            Ok(c) => c,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };
        match client.get_fpx_direct_debit(&params.id).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e)),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for BayarcashMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Bayarcash Payment Gateway MCP Server. Provides tools for creating payments, querying transactions, verifying callbacks, and managing FPX Direct Debit mandates.".into()),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load();
    let server = BayarcashMcp::new(config);

    server
        .serve(tokio::io::join(tokio::io::stdin(), tokio::io::stdout()))
        .await?
        .waiting()
        .await?;

    Ok(())
}
```

**Step 2: Verify it compiles**

Run: `cargo check --bin bayarcash-mcp`
Expected: SUCCESS (may need minor adjustments to rmcp imports based on version)

**Step 3: Commit**

```bash
git add src/bin/mcp.rs
git commit -m "feat: add bayarcash-mcp MCP server with 13 tools"
```

---

### Task 5: Add Serialize to Response Types

**Files:**
- Modify: `~/rust/bayarcash-sdk/src/types/payment.rs`
- Modify: `~/rust/bayarcash-sdk/src/types/transaction.rs`
- Modify: `~/rust/bayarcash-sdk/src/types/bank.rs`
- Modify: `~/rust/bayarcash-sdk/src/types/portal.rs`
- Modify: `~/rust/bayarcash-sdk/src/types/direct_debit.rs`

**Step 1:** Add `Serialize` derive to all response types that need JSON output.

For each file, change `#[derive(Debug, Clone, Deserialize)]` to `#[derive(Debug, Clone, Serialize, Deserialize)]` on these types:

- `PaymentIntent` (payment.rs)
- `Transaction`, `PaginationMeta`, `PaginatedResponse<T>` (transaction.rs)
- `FpxBank` (bank.rs)
- `Portal`, `PaymentChannelInfo` (portal.rs)
- `FpxDirectDebitApplication`, `FpxDirectDebit` (direct_debit.rs)

**Step 2: Verify compilation**

Run: `cargo check`
Expected: SUCCESS

**Step 3: Run all tests**

Run: `cargo test`
Expected: All 15 tests PASS

**Step 4: Commit**

```bash
git add src/types/
git commit -m "feat: add Serialize derive to response types for JSON output"
```

---

### Task 6: Update README and Cargo.toml metadata

**Files:**
- Modify: `~/rust/bayarcash-sdk/README.md`
- Modify: `~/rust/bayarcash-sdk/Cargo.toml`

**Step 1:** Add CLI and MCP server sections to README:
- Installation: `cargo install bayarcash-sdk` (installs both binaries)
- CLI usage examples
- MCP server config for Claude Desktop / claude_desktop_config.json:
```json
{
  "mcpServers": {
    "bayarcash": {
      "command": "bayarcash-mcp",
      "env": {
        "BAYARCASH_TOKEN": "your_token",
        "BAYARCASH_SECRET_KEY": "your_secret"
      }
    }
  }
}
```

**Step 2: Run final checks**

Run: `cargo test && cargo clippy -- -D warnings && cargo fmt`

**Step 3: Commit**

```bash
git add README.md Cargo.toml
git commit -m "docs: update README with CLI and MCP server usage"
```

---

## Summary

| Task | Description |
|------|-------------|
| 1 | Add dependencies and binary targets |
| 2 | Config module (file + env vars) |
| 3 | CLI with clap (full surface) |
| 4 | MCP server with rmcp (13 tools) |
| 5 | Add Serialize to response types |
| 6 | Update README + final polish |
