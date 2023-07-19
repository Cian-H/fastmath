#![allow(unused_imports)]
extern crate fastmath;

use fastmath::*;
use criterion::{Criterion, BenchmarkGroup, measurement::WallTime};
use criterion::{black_box, criterion_group, criterion_main};

fn pow2_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_pow2()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin_fn", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).powi(2)).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin_mul", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x) * x).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_pow2()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin_fn", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).powi(2)).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin_mul", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x) * x).collect::<Vec<f32>>())
    });
}

fn exp_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_exp()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).exp()).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_exp()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).exp()).collect::<Vec<f32>>())
    });
}

fn cos_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_lookup", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).lookup_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).cos()).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_lookup", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).lookup_cos()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).cos()).collect::<Vec<f32>>())
    });
}

fn sigmoid_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_sigmoid()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| sigmoid_builtin_f64(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_sigmoid()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| sigmoid_builtin_f32(black_box(x))).collect::<Vec<f32>>())
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // Prepare x values for testing functions
    let x_f64 = (-10000..10000)
        .map(|a| (a as f64) / 1000.)
        .collect::<Vec<f64>>();
    let x_f32 = (-10000..10000)
        .map(|a| (a as f32) / 1000.)
        .collect::<Vec<f32>>();
    // to ensure tests are fair, we need to instantiate the lookup tables
    1.0f64.lookup_cos();
    1.0f32.lookup_cos();
    // Then, tests can begin
    let mut group = c.benchmark_group("pow2");
    pow2_benchmarks(&mut group, &x_f64, &x_f32);
    group.finish();

    let mut group = c.benchmark_group("exp");
    exp_benchmarks(&mut group, &x_f64, &x_f32);
    group.finish();

    let mut group = c.benchmark_group("cos");
    cos_benchmarks(&mut group, &x_f64, &x_f32);
    group.finish();

    let mut group = c.benchmark_group("sigmoid");
    sigmoid_benchmarks(&mut group, &x_f64, &x_f32);
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

