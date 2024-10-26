//! Cryptographic operations for the Pact blockchain
//!
//! This module provides cryptographic functionality required for interacting with
//! the Kadena blockchain, including key generation, signing, and verification.
//!
//! ## Key Features
//!
//! - ED25519 key pair generation and management
//! - Message signing and signature verification
//! - Blake2b hashing
//! - Hex and Base64URL encoding utilities
//!
//! ## Examples
//!
//! ### Key Generation and Signing
//!
//! ```
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
//! ### Working with Existing Keys
//!
//! ```
//! use kadena::crypto::PactKeypair;
//!
//! // Restore a keypair from an existing secret key
//! let keypair = PactKeypair::generate();
//! let restored = PactKeypair::from_secret_key(&keypair.secret_key()).unwrap();
//! assert_eq!(keypair.public_key(), restored.public_key());
//! ```
//!
//! ## Security Considerations
//!
//! This module uses the following well-audited cryptographic implementations:
//!
//! - `ed25519-dalek` for ED25519 operations
//! - `blake2` for hashing
//! - `rand` for secure random number generation
//!
//! Secret keys should be handled with care and never exposed or logged.

pub mod crypto_error;
pub mod encoding;
pub mod keypair;

pub use crypto_error::*;
pub use encoding::*;
pub use keypair::*;
