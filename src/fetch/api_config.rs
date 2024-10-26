/// Configuration for API client
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Base URL for the API
    pub host: String,
    /// Timeout for requests in seconds
    pub timeout: u64,
    /// Optional API key
    pub api_key: Option<String>,
}

impl ApiConfig {
    /// Create a new API configuration with the required host URL
    ///
    /// # Arguments
    ///
    /// * `host` - Base URL for the API
    /// * `network` - Kadena network
    /// * `chain_id` - Kadena chain id
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::fetch::ApiConfig;
    ///
    /// let config = ApiConfig::new("https://api.testnet.chainweb.com", "testnet04", "0");
    /// ```
    pub fn new(base_url: &str, network: &str, chain_id: &str) -> Self {
        Self {
            host: format!(
                "{}/chainweb/0.0/{}/chain/{}/pact",
                base_url, network, chain_id
            ),
            timeout: 30,
            api_key: None,
        }
    }

    /// Set a custom timeout
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout = seconds;
        self
    }

    /// Set an API key
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
}
