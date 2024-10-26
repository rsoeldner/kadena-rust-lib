// benches/crypto_benchmarks.rs

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kadena::crypto::{
    encoding::{base64url_decode, base64url_encode, bin_to_hex, hex_to_bin},
    hash, PactKeypair,
};

/// Creates a test keypair with known values
fn get_test_keypair() -> PactKeypair {
    PactKeypair::generate()
}

fn benchmark_keypair_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Keypair Operations");

    // Benchmark keypair generation
    group.bench_function("generate_keypair", |b| {
        b.iter(|| PactKeypair::generate);
    });

    // Benchmark keypair restoration
    let keypair = get_test_keypair();
    group.bench_function("restore_keypair", |b| {
        b.iter(|| PactKeypair::from_secret_key(keypair.secret_key()));
    });

    group.finish();
}

fn benchmark_signing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Signing Operations");
    let keypair = get_test_keypair();

    // Test different message sizes
    let message_sizes = [32, 64, 128, 256, 512, 1024, 2048];

    for size in message_sizes {
        let test_message = vec![0u8; size];
        group.bench_with_input(BenchmarkId::new("sign_message", size), &size, |b, _| {
            b.iter(|| keypair.sign(&test_message));
        });
    }

    group.finish();
}

fn benchmark_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("Verification Operations");
    let keypair = get_test_keypair();

    // Test different message sizes
    let message_sizes = [32, 64, 128, 256, 512, 1024, 2048];

    for size in message_sizes {
        let test_message = vec![0u8; size];
        let signature = keypair.sign(&test_message).unwrap();

        group.bench_with_input(BenchmarkId::new("verify_signature", size), &size, |b, _| {
            b.iter(|| keypair.verify(&test_message, &signature));
        });
    }

    group.finish();
}

fn benchmark_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hashing Operations");

    // Test different input sizes
    let input_sizes = [32, 64, 128, 256, 512, 1024, 2048, 4096];

    for size in input_sizes {
        let test_data = vec![0u8; size];
        group.bench_with_input(BenchmarkId::new("hash", size), &size, |b, _| {
            b.iter(|| hash(&test_data));
        });
    }

    group.finish();
}

fn benchmark_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encoding Operations");

    // Test different input sizes
    let input_sizes = [32, 64, 128, 256, 512];

    for size in input_sizes {
        let test_data = vec![0u8; size];
        let hex_data = bin_to_hex(&test_data);
        let base64_data = base64url_encode(&test_data);

        // Benchmark hex encoding
        group.bench_with_input(BenchmarkId::new("hex_encode", size), &size, |b, _| {
            b.iter(|| bin_to_hex(&test_data));
        });

        // Benchmark hex decoding
        group.bench_with_input(BenchmarkId::new("hex_decode", size), &size, |b, _| {
            b.iter(|| hex_to_bin(&hex_data));
        });

        // Benchmark base64url encoding
        group.bench_with_input(BenchmarkId::new("base64url_encode", size), &size, |b, _| {
            b.iter(|| base64url_encode(&test_data));
        });

        // Benchmark base64url decoding
        group.bench_with_input(BenchmarkId::new("base64url_decode", size), &size, |b, _| {
            b.iter(|| base64url_decode(&base64_data));
        });
    }

    group.finish();
}

fn benchmark_real_world_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("Real World Scenarios");
    let keypair = get_test_keypair();

    // Simulate a typical transaction payload
    let typical_payload = r#"{"meta":{"chainId":"0","gasLimit":1500,"gasPrice":0.00000001,"sender":"k:abc123","ttl":3600},"networkId":"testnet04","nonce":"2024-02-20","payload":{"exec":{"code":"(coin.transfer)","data":{}}}}"#;
    let typical_payload_bytes = typical_payload.as_bytes();

    // Benchmark complete transaction signing process
    group.bench_function("complete_transaction_signing", |b| {
        b.iter(|| {
            let msg_hash = hash(typical_payload_bytes);
            let hash_bytes = base64url_decode(&msg_hash).unwrap();
            keypair.sign(&hash_bytes)
        });
    });

    // Benchmark complete verification process
    group.bench_function("complete_transaction_verification", |b| {
        let msg_hash = hash(typical_payload_bytes);
        let hash_bytes = base64url_decode(&msg_hash).unwrap();
        let signature = keypair.sign(&hash_bytes).unwrap();

        b.iter(|| keypair.verify(&hash_bytes, &signature));
    });

    // Benchmark chain of operations
    group.bench_function("chain_of_operations", |b| {
        b.iter(|| {
            let msg_hash = hash(typical_payload_bytes);
            let hash_bytes = base64url_decode(&msg_hash).unwrap();
            let signature = keypair.sign(&hash_bytes).unwrap();
            let verification = keypair.verify(&hash_bytes, &signature).unwrap();
            assert!(verification);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_keypair_operations,
    benchmark_signing,
    benchmark_verification,
    benchmark_hashing,
    benchmark_encoding,
    benchmark_real_world_scenarios,
);
criterion_main!(benches);
