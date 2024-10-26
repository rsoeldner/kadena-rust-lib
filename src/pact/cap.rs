use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// A capability that can be granted to a signer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cap {
    pub name: String,
    pub args: Vec<Value>,
}

impl Cap {
    /// Creates a new capability without arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::pact::Cap;
    ///
    /// let cap = Cap::new("coin.GAS");
    /// assert_eq!(cap.name, "coin.GAS");
    /// assert!(cap.args.is_empty());
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            args: Vec::new(),
        }
    }

    /// Creates a new capability with arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::pact::Cap;
    /// use serde_json::json;
    ///
    /// let cap = Cap::with_args("coin.TRANSFER", vec![
    ///     json!("k:abc123"),
    ///     json!("k:def456"),
    ///     json!(10.0),
    /// ]);
    /// ```
    pub fn with_args(name: &str, args: Vec<Value>) -> Self {
        Self {
            name: name.to_string(),
            args,
        }
    }

    /// Adds an argument to the capability
    pub fn add_arg<T: Into<Value>>(mut self, arg: T) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Creates a transfer capability
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::pact::Cap;
    ///
    /// let transfer_cap = Cap::transfer("k:abc123", "k:def456", 10.0);
    /// assert_eq!(transfer_cap.name, "coin.TRANSFER");
    /// ```
    pub fn transfer(from: &str, to: &str, amount: f64) -> Self {
        Self::with_args("coin.TRANSFER", vec![json!(from), json!(to), json!(amount)])
    }
}
