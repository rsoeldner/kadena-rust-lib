use kadena::crypto::PactKeypair;
use kadena::fetch::{ApiClient, ApiConfig};
use kadena::pact::{cap::Cap, command::Cmd, meta::Meta};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting transfer transaction example");

    // Create keypair from known keys
    let keypair = PactKeypair::generate();

    // Format sender account with "k:" prefix
    let sender_account = format!("k:{}", keypair.public_key);

    let network = "testnet04";
    let chain_id = "0";

    // Create metadata using the new constructor and builder pattern
    let meta = Meta::new(chain_id, &sender_account)
        .with_gas_limit(1500)
        .with_gas_price(0.00000001)
        .with_ttl(3600);

    // Create capabilities using the new constructors
    let caps = vec![
        Cap::new("coin.GAS"), // Using the convenience constructor for GAS capability
        Cap::transfer(&sender_account, "Bob", 10.0), // Using the convenience constructor for TRANSFER
    ];

    // Create Pact code for the transaction
    let pact_code = format!("(coin.transfer \"{}\" \"Bob\" 10.0)", sender_account);

    // Prepare the execution command using the new method
    let transaction_cmd = Cmd::prepare_exec(
        &[(&keypair, caps)],
        None, // Let it generate a random nonce
        &pact_code,
        None,
        meta,
        Some(network.to_string()),
    )?;

    println!("Transaction Payload:");
    println!("{}", serde_json::to_string_pretty(&transaction_cmd)?);

    // Send the transaction to the Kadena testnet node
    // Create the client
    let client = ApiClient::new(
        ApiConfig::new("https://api.testnet.chainweb.com", network, chain_id).with_timeout(60),
    );
    let result = client.local(&transaction_cmd).await?;

    println!("\nTransaction Response:");
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
