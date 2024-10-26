//! Pact command creation and management
//!
//! This module provides functionality for creating, signing, and managing Pact
//! commands for interaction with the Kadena blockchain.
//!
//! ## Module Structure
//!
//! - [`meta`] - Transaction metadata handling
//! - [`cap`] - Capability creation and management
//! - [`command`] - Command preparation and signing
//!
//! ## Examples
//!
//! ### Creating a Simple Transaction
//!
//! ```rust
//! use kadena::pact::{meta::Meta, cap::Cap, command::Cmd};
//! use kadena::crypto::PactKeypair;
//!
//! let keypair = PactKeypair::generate();
//! let sender = format!("k:{}", keypair.public_key());
//!
//! // Create metadata
//! let meta = Meta::new("0", &sender)
//!     .with_gas_limit(1500)
//!     .with_gas_price(0.00000001);
//!
//! // Create capabilities
//! let caps = vec![Cap::new("coin.GAS")];
//!
//! // Prepare command
//! let cmd = Cmd::prepare_exec(
//!     &[(&keypair, caps)],
//!     None,
//!     "(+ 1 2)",
//!     None,
//!     meta,
//!     Some("testnet04".to_string()),
//! ).unwrap();
//! ```

pub mod cap;
pub mod command;
pub mod command_error;
pub mod meta;

pub use cap::*;
pub use command::*;
pub use command_error::*;
pub use meta::*;
