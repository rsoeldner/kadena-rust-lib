use blake2::{digest::consts::U32, Blake2b, Digest};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{encoding, CryptoError};

/// Represents a Pact keypair containing a public key and a secret key
#[derive(Debug, Clone)]
pub struct PactKeypair {
    /// The public key as a hexadecimal string
    pub public_key: String,
    /// The secret key as a hexadecimal string
    pub secret_key: String,
}

impl PactKeypair {
    /// Generate a new ED25519 keypair
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::crypto::PactKeypair;
    ///
    /// let keypair = PactKeypair::generate();
    /// assert_eq!(keypair.public_key().len(), 64); // 32 bytes in hex
    /// ```
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Self {
            public_key: encoding::bin_to_hex(verifying_key.as_bytes()),
            secret_key: encoding::bin_to_hex(&signing_key.to_bytes()),
        }
    }

    /// Restore a keypair from a secret key
    ///
    /// # Arguments
    ///
    /// * `seed` - A string slice containing the hexadecimal representation of the secret key
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::crypto::PactKeypair;
    ///
    /// let original = PactKeypair::generate();
    /// let restored = PactKeypair::from_secret_key(&original.secret_key()).unwrap();
    /// assert_eq!(original.public_key(), restored.public_key());
    /// ```
    pub fn from_secret_key(seed: &str) -> Result<Self, CryptoError> {
        let secret_bytes = encoding::hex_to_bin(seed)?;
        if secret_bytes.len() != 32 {
            return Err(CryptoError::InvalidSeedLength);
        }
        let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().unwrap());
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            public_key: encoding::bin_to_hex(verifying_key.as_bytes()),
            secret_key: seed.to_string(),
        })
    }

    /// Get the public key
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// Get the secret key
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }

    /// Sign a message using this keypair
    ///
    /// # Arguments
    ///
    /// * `msg` - A byte slice containing the message to be signed
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::crypto::PactKeypair;
    ///
    /// let keypair = PactKeypair::generate();
    /// let msg = b"Hello, world!";
    /// let signature = keypair.sign(msg).unwrap();
    /// assert_eq!(signature.len(), 128); // 64 bytes in hex
    /// ```
    pub fn sign(&self, msg: &[u8]) -> Result<String, CryptoError> {
        let secret_bytes = encoding::hex_to_bin(&self.secret_key)?;
        let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().unwrap());
        let signature = signing_key.try_sign(msg)?;
        Ok(encoding::bin_to_hex(signature.to_bytes().as_ref()))
    }

    /// Verify a signature using this keypair's public key
    ///
    /// # Arguments
    ///
    /// * `msg` - A byte slice containing the original message
    /// * `signature` - A string slice containing the hexadecimal representation of the signature
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::crypto::PactKeypair;
    ///
    /// let keypair = PactKeypair::generate();
    /// let msg = b"Hello, world!";
    /// let signature = keypair.sign(msg).unwrap();
    /// assert!(keypair.verify(msg, &signature).unwrap());
    /// ```
    pub fn verify(&self, msg: &[u8], signature: &str) -> Result<bool, CryptoError> {
        verify_signature(msg, signature, &self.public_key)
    }
}

/// Compute the Blake2b hash of the input data
///
/// # Arguments
///
/// * `data` - A byte slice containing the data to be hashed
///
/// # Returns
///
/// A String containing the Base64 URL-safe representation of the hash
///
/// # Examples
///
/// ```
/// use kadena::crypto::hash;
///
/// let data = b"Hello, world!";
/// let hashed = hash(data);
/// assert_eq!(hashed.len(), 43); // 256-bit hash encoded in base64
/// ```
pub fn hash(data: &[u8]) -> String {
    type Blake2b256 = Blake2b<U32>;
    let mut hasher = Blake2b256::new();
    hasher.update(data);
    encoding::base64url_encode(&hasher.finalize())
}

/// Verify a signature with a public key
///
/// This is a standalone function for verifying signatures when you only have
/// a public key and don't need a full keypair.
pub fn verify_signature(
    msg: &[u8],
    signature: &str,
    public_key: &str,
) -> Result<bool, CryptoError> {
    let sig_bytes = encoding::hex_to_bin(signature)?;
    let pub_bytes = encoding::hex_to_bin(public_key)?;

    if pub_bytes.len() != 32 {
        return Err(CryptoError::InvalidSeedLength);
    }

    let signature = Signature::from_slice(&sig_bytes)?;
    let verifying_key = VerifyingKey::from_bytes(&pub_bytes.try_into().unwrap())?;
    Ok(verifying_key.verify(msg, &signature).is_ok())
}
