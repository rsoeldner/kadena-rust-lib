//! Network operations for interacting with Kadena nodes
//!
//! This module provides functionality for sending commands to Kadena nodes,
//! both for local execution and blockchain submission.

use std::time::Duration;

use crate::{pact::command::Cmd, ApiConfig, FetchError};
use log::{debug, error};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};

/// API client for interacting with Kadena nodes
#[derive(Debug)]
pub struct ApiClient {
    config: ApiConfig,
    client: Client,
}

impl ApiClient {
    /// Create a new API client with configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::fetch::{ApiClient, ApiConfig};
    ///
    /// let config = ApiConfig::new("https://api.testnet.chainweb.com", "testnet04", "0")
    ///     .with_timeout(60);
    ///
    /// let client = ApiClient::new(config);
    /// ```
    pub fn new(config: ApiConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }
    fn create_payload(&self, cmd: &Cmd) -> Value {
        json!({
            "sigs": cmd.sigs.iter().map(|sig| json!({ "sig": sig.sig })).collect::<Vec<Value>>(),
            "cmd": cmd.cmd,
            "hash": cmd.hash
        })
    }

    /// Execute a command locally without sending to the blockchain
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to execute
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use kadena::fetch::{ApiClient, ApiConfig};
    /// use kadena::pact::command::Cmd;
    ///
    /// let client = ApiClient::new(ApiConfig::new("https://api.testnet.chainweb.com"));
    /// let cmd = Cmd { /* ... */ };
    /// let result = client.local(&cmd).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn local(&self, cmd: &Cmd) -> Result<Value, FetchError> {
        let url = format!("{}/api/v1/local", self.config.host);
        let payload = self.create_payload(cmd);

        debug!(
            "Sending local request to {}: {}",
            url,
            serde_json::to_string_pretty(&payload)?
        );

        self.execute_request(&url, &payload).await
    }

    /// Send a command to the blockchain
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to send
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use kadena::fetch::{ApiClient, ApiConfig};
    /// use kadena::pact::command::Cmd;
    ///
    /// let client = ApiClient::new(ApiConfig::new("https://api.testnet.chainweb.com"));
    /// let cmd = Cmd { /* ... */ };
    /// let result = client.send(&cmd).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, cmd: &Cmd) -> Result<Value, FetchError> {
        let url = format!("{}/api/v1/send", self.config.host);
        let payload = json!({
            "cmds": [self.create_payload(cmd)]
        });

        debug!(
            "Sending transaction to {}: {}",
            url,
            serde_json::to_string_pretty(&payload)?
        );

        self.execute_request(&url, &payload).await
    }

    async fn execute_request(
        &self,
        url: &str,
        payload: &impl Serialize,
    ) -> Result<Value, FetchError> {
        let mut request = self.client.post(url).json(payload);

        if let Some(api_key) = &self.config.api_key {
            request = request.header("X-API-Key", api_key);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let json_response = response.json().await?;
            debug!(
                "Received response: {}",
                serde_json::to_string_pretty(&json_response)?
            );
            Ok(json_response)
        } else {
            let error_text = response.text().await?;
            error!("API error: {}", error_text);
            Err(FetchError::ApiError(error_text))
        }
    }
}
