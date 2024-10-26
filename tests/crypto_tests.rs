use kadena::crypto::*;

#[test]
fn test_keypair_generation_and_restoration() {
    let keypair = PactKeypair::generate();
    let restored = PactKeypair::from_secret_key(keypair.secret_key()).unwrap();
    assert_eq!(keypair.public_key(), restored.public_key());
}

#[test]
fn test_signing_and_verification() {
    let keypair = PactKeypair::generate();
    let msg = b"test message";
    let signature = keypair.sign(msg).unwrap();
    assert!(keypair.verify(msg, &signature).unwrap());
}
