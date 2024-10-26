use serde::{Deserialize, Serialize};

/// Metadata for a Pact command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    pub sender: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: u64,
    #[serde(rename = "gasPrice")]
    pub gas_price: f64,
    pub ttl: u64,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
}

impl Meta {
    /// Creates a new Meta instance with common default values
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain ID where the command will be executed
    /// * `sender` - The sender's account address
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::pact::Meta;
    ///
    /// let meta = Meta::new("0", "k:abc123");
    /// assert_eq!(meta.chain_id, "0");
    /// assert_eq!(meta.gas_limit, 1500);
    /// ```
    pub fn new(chain_id: &str, sender: &str) -> Self {
        Self {
            chain_id: chain_id.to_string(),
            sender: sender.to_string(),
            gas_limit: 1500,       // Common default
            gas_price: 0.00000001, // Common default
            ttl: 3600,             // Default 1 hour
            creation_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Creates a new Meta instance with all parameters specified
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::pact::Meta;
    ///
    /// let meta = Meta::with_params(
    ///     "0",
    ///     "k:abc123",
    ///     2000,
    ///     0.00000001,
    ///     7200,
    ///     1234567890,
    /// );
    /// ```
    pub fn with_params(
        chain_id: &str,
        sender: &str,
        gas_limit: u64,
        gas_price: f64,
        ttl: u64,
        creation_time: u64,
    ) -> Self {
        Self {
            chain_id: chain_id.to_string(),
            sender: sender.to_string(),
            gas_limit,
            gas_price,
            ttl,
            creation_time,
        }
    }

    /// Modifies the gas limit
    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = gas_limit;
        self
    }

    /// Modifies the gas price
    pub fn with_gas_price(mut self, gas_price: f64) -> Self {
        self.gas_price = gas_price;
        self
    }

    /// Modifies the TTL
    pub fn with_ttl(mut self, ttl: u64) -> Self {
        self.ttl = ttl;
        self
    }
}
