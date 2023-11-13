#![allow(unused_imports)]
extern crate fastmath;

use rayon::prelude::*;
use fastmath::*;
use criterion::{Criterion, BenchmarkGroup, measurement::WallTime};
use criterion::{black_box, criterion_group, criterion_main};

pub mod exact {
    include!("../src/tests/accuracy/exact.rs");
}
include!("../src/tests/accuracy/x.rs");

fn pow2_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_pow2()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin_fn", |b| {
        b.iter(|| x_f64.iter().map(|&x| exact::f64::pow2(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_pow2()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin_fn", |b| {
        b.iter(|| x_f32.iter().map(|&x| exact::f32::pow2(black_box(x))).collect::<Vec<f32>>())
    });
}

fn exp_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_exp()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| exact::f64::exp(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_exp()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| exact::f32::exp(black_box(x))).collect::<Vec<f32>>())
    });
}

fn cos_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_fast_par", |b| {
        b.iter(|| x_f64.par_iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_lookup", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).lookup_cos()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_lookup_map", |b| {
        let inputs = x_f64.par_iter().map(|&x| black_box(x)).collect::<Vec<f64>>();
        b.iter(|| inputs.clone().lookup_cos())
    });
    group.bench_function("f64_lookup_par_map", |b| {
        let inputs = x_f64.par_iter().map(|&x| black_box(x)).collect::<Vec<f64>>();
        b.iter(|| inputs.clone().par_lookup_cos())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| exact::f64::cos(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin_par", |b| {
        b.iter(|| x_f64.par_iter().map(|&x| exact::f64::cos(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_fast_par", |b| {
        b.iter(|| x_f32.par_iter().map(|&x| black_box(x).fast_cos()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_lookup", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).lookup_cos()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_lookup_map", |b| {
        let inputs = x_f32.par_iter().map(|&x| black_box(x)).collect::<Vec<f32>>();
        b.iter(|| inputs.clone().lookup_cos())
    });
    group.bench_function("f32_lookup_par_map", |b| {
        let inputs = x_f32.par_iter().map(|&x| black_box(x)).collect::<Vec<f32>>();
        b.iter(|| inputs.clone().par_lookup_cos())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| exact::f32::cos(black_box(x))).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin_par", |b| {
        b.iter(|| x_f32.par_iter().map(|&x| exact::f32::cos(black_box(x))).collect::<Vec<f32>>())
    });
}

fn sin_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_sin()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_lookup", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).lookup_sin()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| exact::f64::sin(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_sin()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_lookup", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).lookup_sin()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| exact::f32::sin(black_box(x))).collect::<Vec<f32>>())
    });
}

fn sigmoid_benchmarks(group: &mut BenchmarkGroup<WallTime>, x_f64: &[f64], x_f32: &[f32]) {
    group.bench_function("f64_fast", |b| {
        b.iter(|| x_f64.iter().map(|&x| black_box(x).fast_sigmoid()).collect::<Vec<f64>>())
    });
    group.bench_function("f64_builtin", |b| {
        b.iter(|| x_f64.iter().map(|&x| exact::f64::sigmoid(black_box(x))).collect::<Vec<f64>>())
    });
    group.bench_function("f32_fast", |b| {
        b.iter(|| x_f32.iter().map(|&x| black_box(x).fast_sigmoid()).collect::<Vec<f32>>())
    });
    group.bench_function("f32_builtin", |b| {
        b.iter(|| x_f32.iter().map(|&x| exact::f32::sigmoid(black_box(x))).collect::<Vec<f32>>())
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // to ensure tests are fair, we need to instantiate the lookup tables
    1.0f64.lookup_cos();
    1.0f32.lookup_cos();
    // Then, tests can begin
    let mut group = c.benchmark_group("pow2");
    pow2_benchmarks(&mut group, &X_F64, &X_F32);
    group.finish();

    let mut group = c.benchmark_group("exp");
    exp_benchmarks(&mut group, &X_F64, &X_F32);
    group.finish();

    let mut group = c.benchmark_group("cos");
    cos_benchmarks(&mut group, &X_F64, &X_F32);
    group.finish();

    let mut group = c.benchmark_group("sin");
    sin_benchmarks(&mut group, &X_F64, &X_F32);
    group.finish();

    let mut group = c.benchmark_group("sigmoid");
    sigmoid_benchmarks(&mut group, &X_F64, &X_F32);
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

