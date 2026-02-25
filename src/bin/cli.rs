use bayarcash_sdk::{AppConfig, PaymentIntentRequest, TransactionQueryParams};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bayarcash", version, about = "Bayarcash Payment Gateway CLI")]
struct Cli {
    /// Use sandbox environment
    #[arg(long, global = true)]
    sandbox: bool,

    /// API version (v2 or v3)
    #[arg(long, global = true)]
    api_version: Option<String>,

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
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| e.to_string())?;
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
    if let Some(ref v) = cli.api_version {
        config.api_version = Some(v.clone());
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
            let template = "token = \"your_api_token_here\"\nsecret_key = \"your_secret_key_here\"\nsandbox = false\napi_version = \"v2\"\n";
            std::fs::write(&path, template).map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::json!({"status": "ok", "path": path.display().to_string()})
            );
            Ok(())
        }

        Commands::Payment { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                PaymentCommands::Create {
                    channel,
                    order,
                    amount,
                    name,
                    email,
                    phone,
                    currency,
                } => {
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
                    let result = client
                        .create_payment_intent(&req)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                PaymentCommands::Get { id } => {
                    let result = client
                        .get_payment_intent(&id)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }

        Commands::Transaction { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                TransactionCommands::Get { id } => {
                    let result = client
                        .get_transaction(&id)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                TransactionCommands::List {
                    order,
                    status,
                    channel,
                    email,
                    reference,
                } => {
                    let params = TransactionQueryParams {
                        order_number: order,
                        status,
                        payment_channel: channel,
                        payer_email: email,
                        exchange_reference_number: reference,
                    };
                    let result = client
                        .get_all_transactions(&params)
                        .await
                        .map_err(|e| e.to_string())?;
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

        Commands::Checksum { command } => match command {
            ChecksumCommands::Payment {
                secret,
                channel,
                order,
                amount,
                name,
                email,
            } => {
                let cs = bayarcash_sdk::checksum::payment_intent(
                    &secret, channel, &order, amount, &name, &email,
                );
                println!("{}", serde_json::json!({"checksum": cs}));
                Ok(())
            }
            ChecksumCommands::Enrollment {
                secret,
                order,
                amount,
                name,
                email,
                phone,
                id_type,
                id_value,
                reason,
                frequency,
            } => {
                let cs = bayarcash_sdk::checksum::fpx_direct_debit_enrollment(
                    &secret, &order, amount, &name, &email, &phone, &id_type, &id_value, &reason,
                    &frequency,
                );
                println!("{}", serde_json::json!({"checksum": cs}));
                Ok(())
            }
            ChecksumCommands::Maintenance {
                secret,
                amount,
                email,
                phone,
                reason,
                frequency,
            } => {
                let cs = bayarcash_sdk::checksum::fpx_direct_debit_maintenance(
                    &secret, amount, &email, &phone, &reason, &frequency,
                );
                println!("{}", serde_json::json!({"checksum": cs}));
                Ok(())
            }
        },

        Commands::Verify { command } => match command {
            VerifyCommands::Transaction { json, secret } => {
                let input = read_json_input(json)?;
                let data: bayarcash_sdk::TransactionCallbackData =
                    serde_json::from_str(&input).map_err(|e| e.to_string())?;
                let valid = bayarcash_sdk::verification::verify_transaction(&data, &secret);
                println!("{}", serde_json::json!({"valid": valid}));
                Ok(())
            }
            VerifyCommands::PreTransaction { json, secret } => {
                let input = read_json_input(json)?;
                let data: bayarcash_sdk::PreTransactionCallbackData =
                    serde_json::from_str(&input).map_err(|e| e.to_string())?;
                let valid = bayarcash_sdk::verification::verify_pre_transaction(&data, &secret);
                println!("{}", serde_json::json!({"valid": valid}));
                Ok(())
            }
            VerifyCommands::ReturnUrl { json, secret } => {
                let input = read_json_input(json)?;
                let data: bayarcash_sdk::ReturnUrlCallbackData =
                    serde_json::from_str(&input).map_err(|e| e.to_string())?;
                let valid = bayarcash_sdk::verification::verify_return_url(&data, &secret);
                println!("{}", serde_json::json!({"valid": valid}));
                Ok(())
            }
            VerifyCommands::DdApproval { json, secret } => {
                let input = read_json_input(json)?;
                let data: bayarcash_sdk::DirectDebitBankApprovalCallbackData =
                    serde_json::from_str(&input).map_err(|e| e.to_string())?;
                let valid =
                    bayarcash_sdk::verification::verify_direct_debit_bank_approval(&data, &secret);
                println!("{}", serde_json::json!({"valid": valid}));
                Ok(())
            }
            VerifyCommands::DdAuthorization { json, secret } => {
                let input = read_json_input(json)?;
                let data: bayarcash_sdk::DirectDebitAuthorizationCallbackData =
                    serde_json::from_str(&input).map_err(|e| e.to_string())?;
                let valid =
                    bayarcash_sdk::verification::verify_direct_debit_authorization(&data, &secret);
                println!("{}", serde_json::json!({"valid": valid}));
                Ok(())
            }
            VerifyCommands::DdTransaction { json, secret } => {
                let input = read_json_input(json)?;
                let data: bayarcash_sdk::DirectDebitTransactionCallbackData =
                    serde_json::from_str(&input).map_err(|e| e.to_string())?;
                let valid =
                    bayarcash_sdk::verification::verify_direct_debit_transaction(&data, &secret);
                println!("{}", serde_json::json!({"valid": valid}));
                Ok(())
            }
        },

        Commands::Mandate { command } => {
            let client = config.build_client().map_err(|e| e.to_string())?;
            match command {
                MandateCommands::Create { json } => {
                    let data: serde_json::Value =
                        serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    let result = client
                        .create_fpx_direct_debit_enrollment(&data)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Update { id, json } => {
                    let data: serde_json::Value =
                        serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    let result = client
                        .create_fpx_direct_debit_maintenance(&id, &data)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Terminate { id, json } => {
                    let data: serde_json::Value =
                        serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    let result = client
                        .create_fpx_direct_debit_termination(&id, &data)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Get { id } => {
                    let result = client
                        .get_fpx_direct_debit(&id)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
                MandateCommands::Transaction { id } => {
                    let result = client
                        .get_fpx_direct_debit_transaction(&id)
                        .await
                        .map_err(|e| e.to_string())?;
                    print_json(&result);
                    Ok(())
                }
            }
        }
    }
}
