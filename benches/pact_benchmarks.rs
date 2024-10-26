// benches/pact_benchmarks.rs

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kadena::{
    crypto::PactKeypair,
    pact::{cap::Cap, command::Cmd, meta::Meta},
};
use serde_json::json;

fn get_test_keypair() -> PactKeypair {
    PactKeypair::generate()
}

fn benchmark_meta_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Meta Creation");

    group.bench_function("new", |b| {
        b.iter(|| {
            Meta::new("0", "k:sender123");
        });
    });

    group.bench_function("with_all_params", |b| {
        b.iter(|| {
            Meta::new("0", "k:sender123")
                .with_gas_limit(2000)
                .with_gas_price(0.00000002)
                .with_ttl(7200);
        });
    });

    group.finish();
}

fn benchmark_cap_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Capability Creation");

    group.bench_function("gas_cap", |b| {
        b.iter(|| Cap::new("coin.GAS"));
    });

    group.bench_function("transfer_cap", |b| {
        b.iter(|| {
            Cap::transfer("k:sender123", "k:receiver456", 10.0);
        });
    });

    group.bench_function("complex_cap", |b| {
        b.iter(|| {
            Cap::new("custom.CAP")
                .add_arg("arg1")
                .add_arg(42)
                .add_arg(json!({"key": "value"}));
        });
    });

    group.finish();
}

fn benchmark_cmd_preparation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Command Preparation");

    let keypair = get_test_keypair();
    let sender = format!("k:{}", keypair.public_key);
    let meta = Meta::new("0", &sender);

    // Simple command benchmark
    group.bench_function("simple_command", |b| {
        b.iter(|| {
            Cmd::prepare_exec(
                &[(&keypair, vec![Cap::new("coin.GAS")])],
                Some("test-nonce"),
                "(+ 1 2)",
                None,
                meta.clone(),
                Some("testnet04".to_string()),
            )
            .unwrap()
        });
    });

    // Transfer command benchmark
    group.bench_function("transfer_command", |b| {
        let caps = vec![Cap::new("coin.GAS"), Cap::transfer(&sender, "Bob", 10.0)];
        let pact_code = format!("(coin.transfer \"{}\" \"Bob\" 10.0)", sender);

        b.iter(|| {
            Cmd::prepare_exec(
                &[(&keypair, caps.clone())],
                Some("test-nonce"),
                &pact_code,
                None,
                meta.clone(),
                Some("testnet04".to_string()),
            )
            .unwrap()
        });
    });

    // Multiple signers benchmark
    group.bench_function("multiple_signers", |b| {
        let keypair2 = get_test_keypair();
        let caps1 = vec![Cap::new("coin.GAS")];
        let caps2 = vec![Cap::new("coin.GAS"), Cap::transfer(&sender, "Bob", 5.0)];

        b.iter(|| {
            Cmd::prepare_exec(
                &[(&keypair, caps1.clone()), (&keypair2, caps2.clone())],
                Some("test-nonce"),
                "(+ 1 2)",
                None,
                meta.clone(),
                Some("testnet04".to_string()),
            )
            .unwrap()
        });
    });

    // Random nonce benchmark
    group.bench_function("random_nonce", |b| {
        let caps = vec![Cap::new("coin.GAS")];

        b.iter(|| {
            Cmd::prepare_exec(
                &[(&keypair, caps.clone())],
                None, // Use random nonce
                "(+ 1 2)",
                None,
                meta.clone(),
                Some("testnet04".to_string()),
            )
            .unwrap()
        });
    });

    group.finish();
}

fn benchmark_command_with_varying_caps(c: &mut Criterion) {
    let mut group = c.benchmark_group("Command with Varying Capabilities");

    let keypair = get_test_keypair();
    let sender = format!("k:{}", keypair.public_key);
    let meta = Meta::new("0", &sender);

    // Benchmark with different numbers of capabilities
    for num_caps in [1, 2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("num_capabilities", num_caps),
            num_caps,
            |b, &num_caps| {
                let caps = (0..num_caps)
                    .map(|i| Cap::new(&format!("custom.CAP{}", i)).add_arg(format!("arg{}", i)))
                    .collect::<Vec<_>>();

                b.iter(|| {
                    Cmd::prepare_exec(
                        &[(&keypair, caps.clone())],
                        Some("test-nonce"),
                        "(+ 1 2)",
                        None,
                        meta.clone(),
                        Some("testnet04".to_string()),
                    )
                    .unwrap()
                });
            },
        );
    }

    group.finish();
}

fn benchmark_complex_json_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("Complex JSON Handling");

    let keypair = get_test_keypair();
    let sender = format!("k:{}", keypair.public_key);
    let meta = Meta::new("0", &sender);

    group.bench_function("complex_env_data", |b| {
        let env_data = json!({
            "nested": {
                "array": [1, 2, 3, 4, 5],
                "object": {
                    "field1": "value1",
                    "field2": 42,
                    "field3": true,
                    "field4": [
                        {"x": 1, "y": 2},
                        {"x": 3, "y": 4}
                    ]
                }
            }
        });

        b.iter(|| {
            Cmd::prepare_exec(
                &[(&keypair, vec![Cap::new("coin.GAS")])],
                Some("test-nonce"),
                "(+ 1 2)",
                Some(env_data.clone()),
                meta.clone(),
                Some("testnet04".to_string()),
            )
            .unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_meta_creation,
    benchmark_cap_creation,
    benchmark_cmd_preparation,
    benchmark_command_with_varying_caps,
    benchmark_complex_json_data,
);
criterion_main!(benches);
