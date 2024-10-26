use kadena::{
    crypto::PactKeypair,
    pact::{cap::Cap, command::Cmd, meta::Meta},
};
use serde_json::json;

mod meta_tests {
    use super::*;

    #[test]
    fn test_meta_creation() {
        let sender = "k:abc123";
        let meta = Meta::new("0", sender);

        assert_eq!(meta.chain_id, "0");
        assert_eq!(meta.sender, sender);
        assert_eq!(meta.gas_limit, 1500); // Default value
        assert_eq!(meta.gas_price, 0.00000001); // Default value
        assert_eq!(meta.ttl, 3600); // Default value
    }

    #[test]
    fn test_meta_builder_pattern() {
        let meta = Meta::new("0", "k:abc123")
            .with_gas_limit(2000)
            .with_gas_price(0.00000002)
            .with_ttl(7200);

        assert_eq!(meta.gas_limit, 2000);
        assert_eq!(meta.gas_price, 0.00000002);
        assert_eq!(meta.ttl, 7200);
    }
}

mod cap_tests {
    use super::*;

    #[test]
    fn test_transfer_cap_creation() {
        let from = "k:sender123";
        let to = "k:receiver456";
        let amount = 10.0;

        let transfer_cap = Cap::transfer(from, to, amount);

        assert_eq!(transfer_cap.name, "coin.TRANSFER");
        assert_eq!(transfer_cap.args.len(), 3);
        assert_eq!(transfer_cap.args[0], json!(from));
        assert_eq!(transfer_cap.args[1], json!(to));
        assert_eq!(transfer_cap.args[2], json!(amount));
    }

    #[test]
    fn test_cap_builder_pattern() {
        let cap = Cap::new("custom.CAP")
            .add_arg("arg1")
            .add_arg(42)
            .add_arg(json!({"key": "value"}));

        assert_eq!(cap.name, "custom.CAP");
        assert_eq!(cap.args.len(), 3);
        assert_eq!(cap.args[0], json!("arg1"));
        assert_eq!(cap.args[1], json!(42));
        assert_eq!(cap.args[2], json!({"key": "value"}));
    }
}

mod command_tests {
    use super::*;

    #[test]
    fn test_prepare_exec_cmd() {
        let keypair = PactKeypair::generate();
        let sender = format!("k:{}", keypair.public_key);

        let meta = Meta::new("0", &sender);

        let caps = vec![Cap::new("coin.GAS"), Cap::transfer(&sender, "Bob", 10.0)];

        let pact_code = format!("(coin.transfer \"{}\" \"Bob\" 10.0)", sender);

        let cmd = Cmd::prepare_exec(
            &[(&keypair, caps)],
            Some("test-nonce"),
            &pact_code,
            None,
            meta,
            Some("testnet04".to_string()),
        )
        .unwrap();

        // Verify command structure
        assert!(!cmd.hash.is_empty());
        assert_eq!(cmd.sigs.len(), 1);

        // Parse and verify command JSON
        let cmd_json: serde_json::Value = serde_json::from_str(&cmd.cmd).unwrap();

        assert_eq!(cmd_json["nonce"], "test-nonce");
        assert_eq!(cmd_json["networkId"], "testnet04");

        let signer = &cmd_json["signers"][0];
        assert_eq!(signer["scheme"], "ED25519");
        assert_eq!(signer["pubKey"], keypair.public_key);

        // Verify capabilities
        let clist = &signer["clist"];
        assert_eq!(clist.as_array().unwrap().len(), 2);
        assert_eq!(clist[0]["name"], "coin.GAS");
        assert_eq!(clist[1]["name"], "coin.TRANSFER");
    }

    // #[test]
    // fn test_prepare_exec_cmd_to_sign() {
    //     let public_key = "5ea2aa347593e8907b0b4de4698a99a57a88dcbc762916688c640ec8efc05f9f";
    //     let sender = format!("k:{}", public_key);

    //     let meta = Meta::new("0", &sender).with_creation_time(get_current_time());

    //     let caps = vec![Cap::gas()];

    //     let cmd = Cmd::prepare_exec_to_sign(
    //         &[(&public_key.to_string(), caps)],
    //         Some("test-nonce"),
    //         "(+ 1 2)",
    //         None,
    //         meta,
    //         Some("testnet04".to_string()),
    //     )
    //     .unwrap();

    //     assert!(!cmd.hash.is_empty());
    //     assert!(cmd.sigs.is_empty());

    //     let cmd_json: serde_json::Value = serde_json::from_str(&cmd.cmd).unwrap();
    //     assert_eq!(cmd_json["nonce"], "test-nonce");
    //     assert_eq!(cmd_json["networkId"], "testnet04");
    //     assert_eq!(cmd_json["signers"][0]["scheme"], "ED25519");
    //     assert_eq!(cmd_json["signers"][0]["pubKey"], public_key);
    // }

    #[test]
    fn test_random_nonce_generation() {
        let keypair = PactKeypair::generate();
        let sender = format!("k:{}", keypair.public_key);
        let meta = Meta::new("0", &sender);
        let caps = vec![Cap::new("coin.GAS")];

        let cmd1 = Cmd::prepare_exec(
            &[(&keypair, caps.clone())],
            None,
            "(+ 1 2)",
            None,
            meta.clone(),
            Some("testnet04".to_string()),
        )
        .unwrap();

        let cmd2 = Cmd::prepare_exec(
            &[(&keypair, caps)],
            None,
            "(+ 1 2)",
            None,
            meta,
            Some("testnet04".to_string()),
        )
        .unwrap();

        let cmd1_json: serde_json::Value = serde_json::from_str(&cmd1.cmd).unwrap();
        let cmd2_json: serde_json::Value = serde_json::from_str(&cmd2.cmd).unwrap();

        assert_ne!(cmd1_json["nonce"], cmd2_json["nonce"]);
    }

    #[test]
    fn test_multiple_signers() {
        let keypair1 = PactKeypair::generate();
        let keypair2 = PactKeypair::generate(); // Generate a random second keypair

        let sender = format!("k:{}", keypair1.public_key);
        let meta = Meta::new("0", &sender);

        let caps1 = vec![Cap::new("coin.GAS")];
        let caps2 = vec![Cap::new("test.TEST")];

        let cmd = Cmd::prepare_exec(
            &[(&keypair1, caps1), (&keypair2, caps2)],
            Some("test-nonce"),
            "(+ 1 2)",
            None,
            meta,
            Some("testnet04".to_string()),
        )
        .unwrap();

        assert_eq!(cmd.sigs.len(), 2);

        let cmd_json: serde_json::Value = serde_json::from_str(&cmd.cmd).unwrap();
        let signers = cmd_json["signers"].as_array().unwrap();
        assert_eq!(signers.len(), 2);
        assert_eq!(signers[0]["pubKey"], keypair1.public_key);
        assert_eq!(signers[1]["pubKey"], keypair2.public_key);
    }

    #[test]
    fn test_complex_capabilities() {
        let keypair = PactKeypair::generate();
        let sender = format!("k:{}", keypair.public_key);

        // Create a complex capability with nested JSON
        let complex_cap = Cap::new("custom.CAP").add_arg(json!({
            "nested": {
                "array": [1, 2, 3],
                "string": "value",
                "number": 42.5
            }
        }));

        let meta = Meta::new("0", &sender);
        let caps = vec![Cap::new("coin.GAS"), complex_cap];

        let cmd = Cmd::prepare_exec(
            &[(&keypair, caps)],
            Some("test-nonce"),
            "(+ 1 2)",
            None,
            meta,
            Some("testnet04".to_string()),
        )
        .unwrap();

        let cmd_json: serde_json::Value = serde_json::from_str(&cmd.cmd).unwrap();
        let clist = &cmd_json["signers"][0]["clist"];
        assert_eq!(clist.as_array().unwrap().len(), 2);

        // Verify complex capability structure
        let complex = &clist[1];
        assert_eq!(complex["name"], "custom.CAP");
        assert!(complex["args"][0]["nested"]["array"].is_array());
        assert_eq!(complex["args"][0]["nested"]["string"], "value");
        assert_eq!(complex["args"][0]["nested"]["number"], 42.5);
    }
}

// Integration tests to verify module interactions
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_transaction_preparation() {
        let keypair = PactKeypair::generate();
        let sender = format!("k:{}", keypair.public_key);

        // Create metadata
        let meta = Meta::new("0", &sender)
            .with_gas_limit(2000)
            .with_gas_price(0.00000001);

        // Create capabilities
        let caps = vec![Cap::new("coin.GAS"), Cap::transfer(&sender, "Bob", 10.0)];

        // Create Pact code
        let pact_code = format!("(coin.transfer \"{}\" \"Bob\" 10.0)", sender);

        // Prepare command
        let cmd = Cmd::prepare_exec(
            &[(&keypair, caps)],
            Some("test-nonce"),
            &pact_code,
            None,
            meta,
            Some("testnet04".to_string()),
        )
        .unwrap();

        // Verify complete transaction structure
        let cmd_json: serde_json::Value = serde_json::from_str(&cmd.cmd).unwrap();

        // Verify metadata
        assert_eq!(cmd_json["meta"]["chainId"], "0");
        assert_eq!(cmd_json["meta"]["sender"], sender);
        assert_eq!(cmd_json["meta"]["gasLimit"], 2000);

        // Verify signer and capabilities
        let signer = &cmd_json["signers"][0];
        assert_eq!(signer["pubKey"], keypair.public_key);
        assert_eq!(signer["clist"][0]["name"], "coin.GAS");
        assert_eq!(signer["clist"][1]["name"], "coin.TRANSFER");

        // Verify signatures
        assert!(!cmd.sigs.is_empty());
        assert!(!cmd.hash.is_empty());
    }
}
