use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Instant;

fn integer_arithmetic_benchmark() {
    let start = Instant::now();
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    let duration = start.elapsed();
    println!("Integer Arithmetic: {:?}", duration);
}

fn floating_point_arithmetic_benchmark() {
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..1_000_000 {
        sum += i as f64 * 0.1;
    }
    let duration = start.elapsed();
    println!("Floating-Point Arithmetic: {:?}", duration);
}

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Instant;
use sha2::{Sha256, Digest};

fn integer_arithmetic_benchmark() {
    let start = Instant::now();
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    let duration = start.elapsed();
    println!("Integer Arithmetic: {:?}", duration);
}

fn floating_point_arithmetic_benchmark() {
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..1_000_000 {
        sum += i as f64 * 0.1;
    }
    let duration = start.elapsed();
    println!("Floating-Point Arithmetic: {:?}", duration);
}

fn cryptography_benchmark() {
    let start = Instant::now();
    let mut hasher = Sha256::new();
    hasher.update(b"some data");
    let _result = hasher.finalize();
    let duration = start.elapsed();
    println!("Cryptography: {:?}", duration);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("integer_arithmetic", |b| b.iter(|| integer_arithmetic_benchmark()));
    c.bench_function("floating_point_arithmetic", |b| b.iter(|| floating_point_arithmetic_benchmark()));
    c.bench_function("cryptography", |b| b.iter(|| cryptography_benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("integer_arithmetic", |b| b.iter(|| integer_arithmetic_benchmark()));
    c.bench_function("floating_point_arithmetic", |b| b.iter(|| floating_point_arithmetic_benchmark()));
    c.bench_function("cryptography", |b| b.iter(|| cryptography_benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Instant;
use sha2::{Sha256, Digest};

fn integer_arithmetic_benchmark() {
    let start = Instant::now();
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    let duration = start.elapsed();
    println!("Integer Arithmetic: {:?}", duration);
}

fn floating_point_arithmetic_benchmark() {
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..1_000_000 {
        sum += i as f64 * 0.1;
    }
    let duration = start.elapsed();
    println!("Floating-Point Arithmetic: {:?}", duration);
}

fn cryptography_benchmark() {
    let start = Instant::now();
    let mut hasher = Sha256::new();
    hasher.update(b"some data");
    let _result = hasher.finalize();
    let duration = start.elapsed();
    println!("Cryptography: {:?}", duration);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("integer_arithmetic", |b| b.iter(|| integer_arithmetic_benchmark()));
    c.bench_function("floating_point_arithmetic", |b| b.iter(|| floating_point_arithmetic_benchmark()));
    c.bench_function("cryptography", |b| b.iter(|| cryptography_benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);