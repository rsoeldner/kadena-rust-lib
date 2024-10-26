# Kadena Rust Lib

![Continuous Integration](https://github.com/ledger-things/kadena-rust-lib/workflows/Continuous%20Integration/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

A comprehensive Rust library for interacting with Pact smart contracts and the Kadena blockchain. This library provides a type-safe, ergonomic interface for creating and signing Pact commands, managing cryptographic operations, and interacting with Kadena nodes.

## Features

- **🔑 Cryptographic Operations**
  - ED25519 key generation and management
  - Message signing and verification
  - Blake2b hashing
- **📜 Pact Command Creation**
  - Type-safe command builders
  - Capability management
  - Transaction metadata handling
- **🔄 Transaction Management**
  - Command preparation
  - Transaction signing
  - Local and Send endpoint integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kadena = "0.1.0"
```

## Quick Start

```rust
use kadena::{
    crypto::PactKeypair,
    pact::{
        meta::Meta,
        cap::Cap,
        command::Cmd,
    },
    fetch::{ApiClient, ApiConfig}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
```

## Documentation

For detailed documentation and examples, visit [docs.rs/kadena](https://docs.rs/kadena).

## Examples

See the [examples](examples/) directory for more detailed examples.

## Performance

This library includes comprehensive benchmarks that you can run with:

```bash
cargo bench
```

View the benchmark results in `target/criterion/report/index.html`.

## Safety and Security

This library uses well-audited cryptographic implementations:

- ED25519 operations via `ed25519-dalek`
- Blake2b hashing via `blake2`
- Secure random number generation via `rand`

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/ledger-things/kadena-rust-lib
   cd kadena
   ```

2. Run tests:

   ```bash
   cargo test
   ```

3. Run benchmarks:
   ```bash
   cargo bench
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.
