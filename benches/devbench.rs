#![allow(dead_code, unused_imports)]

extern crate fastmath;

use fastmath::*;
use criterion::{Criterion, BenchmarkGroup, measurement::WallTime};
use criterion::{black_box, criterion_group, criterion_main};
use std::f32::consts as f32_consts;
use std::f64::consts as f64_consts;

pub mod exact {
    include!("../src/tests/accuracy/exact.rs");
}
include!("../src/tests/accuracy/x.rs");

fn dev_cos(x: f64) -> f64 {
    const ONE: f64 = 1.0;
    let v = ((((x + f64_consts::PI).abs()) % f64_consts::TAU) - f64_consts::PI).abs();
    let qpprox = ONE - f64_consts::FRAC_2_PI * v;
    qpprox + f64_consts::FRAC_PI_6 * qpprox * (ONE - qpprox * qpprox)
}

fn devbench(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("dev_cos", |b| {
        b.iter(|| X_F64.iter().map(|&x| dev_cos(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("fast_cos", |b| {
        b.iter(|| X_F64.iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("lookup_cos", |b| {
        b.iter(|| X_F64.iter().map(|&x| black_box(x).lookup_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("builtin_cos", |b| {
        b.iter(|| X_F64.iter().map(|&x| exact::f64::cos(black_box(x))).collect::<Vec<f64>>())
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // Then, tests can begin
    let mut group = c.benchmark_group("devbench");
    devbench(&mut group);
    group.finish();
}

criterion_group!(devbenches, criterion_benchmark);
criterion_main!(devbenches);