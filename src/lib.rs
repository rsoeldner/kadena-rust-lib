//! # Kadena
//!
//! A comprehensive Rust library for interacting with Pact smart contracts and the Kadena blockchain.
//! This library provides a type-safe, ergonomic interface for creating and signing Pact commands,
//! managing cryptographic operations, and interacting with Kadena nodes.
//!
//! ## Features
//!
//! - **Cryptographic Operations**: Key generation, signing, and verification using ED25519
//! - **Pact Command Creation**: Type-safe builders for creating and signing Pact commands
//! - **Capability Management**: Create and manage Pact capabilities
//! - **Transaction Management**: Create, sign, and send transactions to Kadena nodes
//!
//! ## Quick Start
//!
//! ```rust
//! use kadena::{
//!     crypto::PactKeypair,
//!     pact::{
//!         meta::Meta,
//!         cap::Cap,
//!         command::Cmd,
//!     },
//!     fetch::{ApiClient, ApiConfig}
//! };
//!
//! // Generate a new keypair
//! let keypair = PactKeypair::generate();
//!
//! // Set network and chain id
//! let network = "testnet04";
//! let chain_id = "0";
//!
//! // Create metadata for the transaction
//! let meta = Meta::new(chain_id, &format!("k:{}", keypair.public_key()))
//!     .with_gas_limit(1500)
//!     .with_gas_price(0.00000001);
//!
//! // Create capabilities
//! let caps = vec![
//!     Cap::new("coin.GAS"),
//!     Cap::transfer(
//!         &format!("k:{}", keypair.public_key()),
//!         "k:receiver",
//!         10.0
//!     ),
//! ];
//!
//! // Prepare the command
//! let cmd = Cmd::prepare_exec(
//!     &[(&keypair, caps)],
//!     Vec::new(),
//!     None,
//!     "(coin.transfer)",
//!     None,
//!     meta,
//!     Some(network.to_string()),
//! ).unwrap();
//!
//!  // Create the client
//! let client = ApiClient::new(
//!    ApiConfig::new("https://api.testnet.chainweb.com", network, chain_id).with_timeout(60),
//! );
//!
//! //Send the tx
//! //let result = client.local(&transaction_cmd).await?;
//! ```
//!
//! ## Modules
//!
//! - [`crypto`] - Cryptographic operations and key management
//! - [`pact`] - Pact command creation and management
//!   - [`pact::meta`] - Transaction metadata handling
//!   - [`pact::cap`] - Capability creation and management
//!   - [`pact::command`] - Command preparation and signing
//! - [`fetch`] - API client management
//!
//! ## Examples
//!
//! ### Key Management
//!
//! ```rust
//! use kadena::crypto::PactKeypair;
//!
//! // Generate a new keypair
//! let keypair = PactKeypair::generate();
//!
//! // Sign a message
//! let message = b"Hello, Kadena!";
//! let signature = keypair.sign(message).unwrap();
//!
//! // Verify the signature
//! assert!(keypair.verify(message, &signature).unwrap());
//! ```
//!
//! ### Creating a Transfer Transaction
//!
//! ```rust
//! use kadena::pact::{
//!     meta::Meta,
//!     cap::Cap,
//!     command::Cmd,
//! };
//! use kadena::crypto::PactKeypair;
//!
//! // Set up the transaction
//! let keypair = PactKeypair::generate();
//! let sender = format!("k:{}", keypair.public_key());
//!
//! let meta = Meta::new("0", &sender);
//! let caps = vec![
//!     Cap::new("coin.GAS"),
//!     Cap::transfer(&sender, "k:receiver", 10.0),
//! ];
//!
//! let pact_code = format!(
//!     "(coin.transfer \"{}\" \"k:receiver\" 10.0)",
//!     sender
//! );
//!
//! let cmd = Cmd::prepare_exec(
//!     &[(&keypair, caps)],
//!     Vec::new(),
//!     None,
//!     &pact_code,
//!     None,
//!     meta,
//!     Some("testnet04".to_string()),
//! ).unwrap();
//! ```
//!
//! ## Performance
//!
//! This library includes comprehensive benchmarks for all critical operations. You can run them with:
//!
//! ```bash
//! cargo bench
//! ```
//!
//! ## Safety and Security
//!
//! This library uses well-audited cryptographic implementations:
//!
//! - ED25519 operations via `ed25519-dalek`
//! - Blake2b hashing via `blake2`
//! - Secure random number generation via `rand`
//!
//! ## Error Handling
//!
//! All operations that can fail return `Result` types with specific error enums:
//!
//! ```rust
//! use kadena::crypto::{PactKeypair, CryptoError};
//!
//! fn handle_signing() -> Result<(), CryptoError> {
//!     let keypair = PactKeypair::generate();
//!     let signature = keypair.sign(b"message")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Contribution
//!
//! Contributions are welcome! Please feel free to submit a Pull Request. Check out our
//! [Contribution Guidelines](CONTRIBUTING.md) for more information.
//!
//! ## License
//!
//! This project is licensed under the MIT License.
//!
pub mod crypto;
pub mod fetch;
pub mod pact;

pub use crypto::*;
pub use fetch::*;
pub use pact::*;
