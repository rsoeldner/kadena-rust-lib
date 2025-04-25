use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{base64url_decode, cap::Cap, hash, meta::Meta, CommandError, PactKeypair};

/// Implementation for SignaturePayload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignaturePayload {
    pub sig: String,
}

impl SignaturePayload {
    pub fn new(sig: String) -> Self {
        Self { sig }
    }
}

/// Implementation for CommandSigner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSigner {
    pub scheme: String,
    #[serde(rename = "pubKey")]
    pub pub_key: String,
    pub clist: Vec<Cap>,
}

impl CommandSigner {
    /// Creates a new ED25519 signer
    pub fn new_ed25519(pub_key: &str, caps: Vec<Cap>) -> Self {
        Self {
            scheme: "ED25519".to_string(),
            pub_key: pub_key.to_string(),
            clist: caps,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandVerifier {
    pub name: String,
    pub proof: String,
    pub clist: Vec<Cap>
}

impl CommandVerifier {
    pub fn new_verifier(name: &str, proof: &str, caps: Vec<Cap>) -> Self {
        Self { name: name.to_string(), proof: proof.to_string(), clist: caps }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecPayload {
    pub exec: ExecCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecCommand {
    pub code: String,
    pub data: Value,
}

impl ExecCommand {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            data: json!({}),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPayload {
    pub nonce: String,
    pub meta: Meta,
    pub signers: Vec<CommandSigner>,
    pub verifiers: Vec<CommandVerifier>,
    #[serde(rename = "networkId")]
    pub network_id: Option<String>,
    pub payload: ExecPayload,
}

impl CommandPayload {
    pub fn new(meta: Meta) -> Self {
        Self {
            nonce: generate_random_nonce(),
            meta,
            signers: Vec::new(),
            verifiers: Vec::new(),
            network_id: None,
            payload: ExecPayload::default(),
        }
    }

    pub fn with_nonce(mut self, nonce: String) -> Self {
        self.nonce = nonce;
        self
    }

    pub fn with_network_id(mut self, network_id: impl Into<String>) -> Self {
        self.network_id = Some(network_id.into());
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.payload.exec.code = code.into();
        self
    }

    pub fn with_signers(mut self, signers: Vec<CommandSigner>) -> Self {
        self.signers = signers;
        self
    }

    pub fn with_verifiers(mut self, verifiers: Vec<CommandVerifier>) -> Self {
        self.verifiers = verifiers;
        self
    }

    pub fn with_env_data(mut self, data: Value) -> Self {
        self.payload.exec.data = data;
        self
    }

    pub fn add_signer(mut self, signer: CommandSigner) -> Self {
        self.signers.push(signer);
        self
    }


    pub fn add_verifier(mut self, verifier: CommandVerifier) -> Self {
        self.verifiers.push(verifier);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmd {
    /// List of signatures for the command
    pub sigs: Vec<SignaturePayload>,
    /// The serialized command string
    pub cmd: String,
    /// The command hash
    pub hash: String,
}

impl Cmd {
    /// Prepares an execution command with signatures
    ///
    /// # Arguments
    ///
    /// * `signers` - A slice of tuples containing keypairs and their associated capabilities
    /// * `nonce` - Optional nonce value, if not provided a random one will be generated
    /// * `pact_code` - The Pact code to execute
    /// * `env_data` - Optional environment data for the execution
    /// * `meta` - Metadata for the command
    /// * `network_id` - Optional network identifier
    ///
    /// # Returns
    ///
    /// Returns a Result containing the prepared Cmd if successful, or a CommandError if preparation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use kadena::pact::{Cmd, Meta, Cap};
    /// use kadena::crypto::PactKeypair;
    ///
    /// let keypair = PactKeypair::generate();
    /// let meta = Meta::new("0", &format!("k:{}", keypair.public_key));
    /// let caps = vec![Cap::new("coin.GAS")];
    ///
    /// let cmd = Cmd::prepare_exec(
    ///     &[(&keypair, caps)],
    ///     Vec::new(),
    ///     None,
    ///     "(+ 1 2)",
    ///     None,
    ///     meta,
    ///     Some("testnet04".to_string()),
    /// ).unwrap();
    /// ```
    pub fn prepare_exec(
        signers: &[(&PactKeypair, Vec<Cap>)],
        verifiers: Vec<CommandVerifier>,
        nonce: Option<&str>,
        pact_code: &str,
        env_data: Option<Value>,
        meta: Meta,
        network_id: Option<String>,
    ) -> Result<Self, CommandError> {
        // Create signers
        let signers_data: Vec<CommandSigner> = signers
            .iter()
            .map(|(kp, caps)| CommandSigner::new_ed25519(&kp.public_key, caps.clone()))
            .collect();

        // Create command payload
        let command_payload = CommandPayload::new(meta)
            .with_nonce(
                nonce
                    .map(ToString::to_string)
                    .unwrap_or_else(generate_random_nonce),
            )
            .with_code(pact_code.to_string())
            .with_signers(signers_data)
            .with_verifiers(verifiers);

        // Add optional fields
        let command_payload = if let Some(network_id) = network_id {
            command_payload.with_network_id(network_id)
        } else {
            command_payload
        };

        let command_payload = if let Some(data) = env_data {
            command_payload.with_env_data(data)
        } else {
            command_payload
        };

        // Serialize and hash
        let cmd = serde_json::to_string(&command_payload)?;
        let cmd_hash = hash(cmd.as_bytes());

        // Create signatures
        let hash_bytes = base64url_decode(&cmd_hash)?;
        let sigs = signers
            .iter()
            .filter_map(|(kp, _)| {
                kp.sign(&hash_bytes)
                    .map(|sig| -> SignaturePayload { SignaturePayload::new(sig) })
                    .ok()
            })
            .collect();

        Ok(Self {
            hash: cmd_hash,
            sigs,
            cmd,
        })
    }
}

/// Generates a random nonce for a command.
fn generate_random_nonce() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    URL_SAFE_NO_PAD.encode(&random_bytes[..24])
}
