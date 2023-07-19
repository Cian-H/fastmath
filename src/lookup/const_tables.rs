// lookup/const_tables.rs

use once_cell::sync::Lazy;
use std::fs::read;
use bincode::deserialize;
use super::lookup_table::*;

pub const SIN_LOOKUP_F32: Lazy<EndoSinLookupTable<f32>> = Lazy::new(|| {
    deserialize(
        &read("src/lookup/data/sin_f32.bin").expect("Failed to read sin_f64.bin")
    ).expect("Failed to load SIN_LOOKUP_F32")
});

pub const SIN_LOOKUP_F64: Lazy<EndoSinLookupTable<f64>> = Lazy::new(|| {
    deserialize(
        &read("src/lookup/data/sin_f64.bin").expect("Failed to read sin_f32.bin")
    ).expect("Failed to load SIN_LOOKUP_F64")
});

pub const COS_LOOKUP_F32: Lazy<EndoCosLookupTable<f32>> = Lazy::new(|| {
    deserialize(
        &read("src/lookup/data/cos_f32.bin").expect("Failed to read cos_f64.bin")
    ).expect("Failed to load COS_LOOKUP_F32")
});

pub const COS_LOOKUP_F64: Lazy<EndoCosLookupTable<f64>> = Lazy::new(|| {
    deserialize(
        &read("src/lookup/data/cos_f64.bin").expect("Failed to read cos_f32.bin")
    ).expect("Failed to load COS_LOOKUP_F64")
});
