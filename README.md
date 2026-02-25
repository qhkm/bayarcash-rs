# bayarcash-rs

Rust SDK for Bayarcash Payment Gateway API.

[![Crates.io](https://img.shields.io/crates/v/bayarcash-rs)](https://crates.io/crates/bayarcash-rs)
[![Docs.rs](https://docs.rs/bayarcash-rs/badge.svg)](https://docs.rs/bayarcash-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Installation

```toml
[dependencies]
bayarcash-rs = "0.2"
tokio = { version = "1", features = ["full"] }
```

Or with `cargo add`:

```sh
cargo add bayarcash-rs
```

## Quick Start

```rust
use bayarcash::{Bayarcash, ApiVersion, PaymentChannel, PaymentIntentRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Bayarcash::builder("your_api_token_here")
        .sandbox(true)
        .api_version(ApiVersion::V3)
        .timeout(Duration::from_secs(60))
        .build()?;

    let request = PaymentIntentRequest {
        payment_channel: PaymentChannel::Fpx as u8,
        order_number: "ORDER-001".to_string(),
        amount: 100.00,
        payer_name: "John Doe".to_string(),
        payer_email: "john@example.com".to_string(),
        payer_telephone_number: Some("+60123456789".to_string()),
        currency: Some("MYR".to_string()),
        callback_url: None,
        return_url: None,
        metadata: None,
        checksum: None,
    };

    let intent = client.create_payment_intent(&request).await?;
    println!("Redirect URL: {}", intent.url);

    Ok(())
}
```

## Payment Channels

| ID | Variant | Description |
|----|---------|-------------|
| 1 | `PaymentChannel::Fpx` | FPX Online Banking |
| 2 | `PaymentChannel::ManualTransfer` | Manual Bank Transfer |
| 3 | `PaymentChannel::FpxDirectDebit` | FPX Direct Debit |
| 4 | `PaymentChannel::FpxLineOfCredit` | FPX Line of Credit |
| 5 | `PaymentChannel::DuitnowDobw` | DuitNow Online Banking/Wallets |
| 6 | `PaymentChannel::DuitnowQr` | DuitNow QR |
| 7 | `PaymentChannel::Spaylater` | SPayLater |
| 8 | `PaymentChannel::BoostPayflex` | Boost PayFlex |
| 9 | `PaymentChannel::Qrisob` | QRIS Online Banking |
| 10 | `PaymentChannel::Qriswallet` | QRIS Wallet |
| 11 | `PaymentChannel::Nets` | NETS |
| 12 | `PaymentChannel::CreditCard` | Credit Card |
| 13 | `PaymentChannel::Alipay` | Alipay |
| 14 | `PaymentChannel::Wechatpay` | WeChat Pay |
| 15 | `PaymentChannel::Promptpay` | PromptPay |

## Features

- **API v2 and v3** - full support for both API versions with compile-time version guard
- **Sandbox and production** - switch environments with `.sandbox(true)`
- **Checksum generation** - HMAC-SHA256 checksums for payment intents and direct debit
- **Callback verification** - verify incoming webhook payloads from Bayarcash
- **Typed errors** - `BayarcashError` variants for 404, 422, 429, timeouts, and version mismatches
- **Async/await** - fully async with `tokio` and `reqwest`

## API Overview

### Client

```rust
// Default client (production, v2, 30s timeout)
let client = Bayarcash::new("token")?;

// Custom configuration
let client = Bayarcash::builder("token")
    .sandbox(true)
    .api_version(ApiVersion::V3)
    .timeout(Duration::from_secs(60))
    .build()?;
```

### Payment Intents (v2 + v3)

```rust
client.create_payment_intent(&request).await?;
client.get_payment_intent("pi_id").await?;   // v3 only
```

### Transactions (v3 only)

```rust
client.get_transaction("tx_id").await?;
client.get_all_transactions(&params).await?;
client.get_transactions_by_order("ORDER-001").await?;
client.get_transactions_by_email("john@example.com").await?;
client.get_transactions_by_status("paid").await?;
client.get_transactions_by_channel(PaymentChannel::Fpx).await?;
client.get_transaction_by_reference("REF-001").await?;
```

### FPX Banks

```rust
let banks = client.fpx_banks_list().await?;
```

### Portals

```rust
let portals = client.get_portals().await?;
let channels = client.get_channels("portal_key").await?;
```

### FPX Direct Debit

```rust
client.create_fpx_direct_debit_enrollment(&data).await?;
client.create_fpx_direct_debit_maintenance("mandate_id", &data).await?;
client.create_fpx_direct_debit_termination("mandate_id", &data).await?;
client.get_fpx_direct_debit("mandate_id").await?;
client.get_fpx_direct_debit_transaction("tx_id").await?;
```

### Manual Bank Transfer

```rust
client.create_manual_bank_transfer(&data, false).await?;
client.update_manual_bank_transfer_status("ref_no", "paid", "100.00").await?;
```

### Checksum Generation

```rust
use bayarcash::checksum;

let cs = checksum::payment_intent(
    "secret_key",
    PaymentChannel::Fpx as u8,
    "ORDER-001",
    100.00,
    "John Doe",
    "john@example.com",
);
```

### Callback Verification

```rust
use bayarcash::verification;

let is_valid = verification::verify_callback("secret_key", &callback_data);
```

For full API documentation, see [docs.rs/bayarcash-rs](https://docs.rs/bayarcash-rs).

## CLI Usage

Install the CLI and MCP server:

```sh
cargo install bayarcash-rs
```

This installs two binaries: `bayarcash` (CLI) and `bayarcash-mcp` (MCP server).

### Configuration

```sh
# Create config file
bayarcash init

# Or set environment variables
export BAYARCASH_TOKEN="your_api_token"
export BAYARCASH_SECRET_KEY="your_secret_key"
export BAYARCASH_SANDBOX=true
export BAYARCASH_API_VERSION=v3
```

Config precedence: CLI flags > environment variables > `~/.bayarcash/config.toml`

### Examples

```sh
# List FPX banks
bayarcash banks list

# Create a payment intent
bayarcash payment create --channel 1 --order ORD-001 --amount 100.00 --name "John" --email "john@example.com"

# Get a transaction
bayarcash transaction get tx_123

# Generate checksum
bayarcash checksum payment --secret sk_test --channel 1 --order ORD-001 --amount 100 --name "John" --email "j@e.com"

# Verify callback (JSON as argument)
bayarcash verify transaction '{"id":"1","checksum":"abc..."}' --secret sk_test

# Use sandbox environment
bayarcash --sandbox banks list

# Use API v3
bayarcash --api-version v3 transaction list --status success
```

All commands output JSON to stdout. Errors go to stderr.

## MCP Server (for AI Agents)

The `bayarcash-mcp` binary exposes Bayarcash operations as MCP tools via stdio transport.

### Claude Desktop Configuration

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

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

### Available Tools

| Tool | Description |
|------|-------------|
| `create_payment` | Create a payment intent |
| `get_payment` | Get payment intent by ID (v3) |
| `get_transaction` | Get transaction by ID |
| `list_transactions` | List transactions with filters (v3) |
| `list_banks` | List FPX banks |
| `list_portals` | List portals |
| `get_channels` | Get portal payment channels |
| `generate_checksum` | Generate HMAC-SHA256 checksum |
| `verify_callback` | Verify callback signature |
| `create_mandate` | Create DD enrollment |
| `update_mandate` | Update DD mandate |
| `get_mandate` | Get mandate details |
| `get_mandate_transaction` | Get mandate transaction |

## License

MIT - see [LICENSE](LICENSE).
