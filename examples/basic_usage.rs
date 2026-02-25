use bayarcash_sdk::{ApiVersion, Bayarcash, PaymentChannel, PaymentIntentRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize client (sandbox mode for testing)
    let client = Bayarcash::builder("your_api_token_here")
        .sandbox(true)
        .api_version(ApiVersion::V3)
        .timeout(Duration::from_secs(60))
        .build()?;

    // Create a payment intent
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

    match client.create_payment_intent(&request).await {
        Ok(intent) => {
            println!("Payment Intent created!");
            println!("  ID: {}", intent.id);
            println!("  Redirect URL: {}", intent.url);
            println!("  Status: {}", intent.status);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Generate a checksum
    let checksum = bayarcash_sdk::checksum::payment_intent(
        "your_secret_key",
        PaymentChannel::Fpx as u8,
        "ORDER-001",
        100.00,
        "John Doe",
        "john@example.com",
    );
    println!("Generated checksum: {}", checksum);

    Ok(())
}
