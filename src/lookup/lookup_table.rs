use std::f32::consts as f32_consts;
use std::f64::consts as f64_consts;
use std::cmp::Ordering;
use num_traits::identities::One;
use num_traits::ops::wrapping::WrappingSub;
use num_traits::float::{Float, FloatConst};
use rayon::iter::{ParallelIterator, IntoParallelIterator};

use crate::{
    impl_fbitfbit_lookup_table,
    impl_cycling_fbitfbit_lookup_table,
};
use crate::lookup::TABLE_SIZE;
use crate::lookup::const_tables::*;

// TODO: Test phf for lookup tables

pub trait ToIterator<T>: IntoIterator<Item = T> {}
impl<T, I> ToIterator<T> for I where I: IntoIterator<Item = T> {}

pub trait ToParallelIterator<T>: IntoParallelIterator<Item = T> {}
impl<T, I> ToParallelIterator<T> for I where I: IntoParallelIterator<Item = T> {}


pub trait ToIndex {
    fn to_index(&self) -> usize;
}
impl ToIndex for f32 {
    fn to_index(&self) -> usize {
        *self as usize
    }
}
impl ToIndex for f64 {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

pub trait FromIndex {
    fn from_index(index: usize) -> Self;
}
impl FromIndex for f32 {
    fn from_index(index: usize) -> Self {
        index as f32
    }
}
impl FromIndex for f64 {
    fn from_index(index: usize) -> Self {
        index as f64
    }
}

#[derive(Debug, Clone)]
pub struct FloatLookupTable<T1, T2>
where 
    T1: Float,
    T2: Float,
{
    keys: [T1; TABLE_SIZE],
    max_key: T1,
    values: [T2; TABLE_SIZE],
}
impl<T1, T2> FloatLookupTable<T1, T2>
where 
    T1: Float + std::marker::Send + std::marker::Sync,
    T1: ToIndex,
    f64: From<T1>,
    T2: Float + std::marker::Send + std::marker::Sync,
{
    pub fn new(keys: [T1; TABLE_SIZE], values: [T2; TABLE_SIZE]) -> Self {
        FloatLookupTable {
            keys: keys,
            max_key: keys[TABLE_SIZE - 1],
            values: values,
        }
    }

    pub fn get_next(&self, key: T1) -> T2
    {
        // Find value at the nearest key, using interpolative search
        // This assumes evenly distributed keys
        let max_index: T1 = T1::from(self.keys.len() - 1).unwrap();
        let index: usize = (key * max_index / self.max_key).ceil().to_index();
        // Ensure value cannot be out of bounds
        *self.values.get(index).unwrap_or_else(
            || -> &T2 {
                if index >= self.values.len() {
                    self.values.last().unwrap()
                } else {
                    panic!("NaN or Inf received or in lookup table")
                }
            }
        )
    }

    pub fn key_to_index(&self, key: T1) -> usize
    {
        (key * T1::from(self.keys.len() - 1).unwrap() / self.max_key).ceil().to_index()
    }

    pub fn lookup(&self, key: T1) -> T2 {
        self.get_next(key)
    }

    pub fn map_lookups<'a, I>(&'a self, keys: I) -> impl Iterator<Item=T2> + 'a
    where
        I: ToIterator<T1> + 'a,
    {
        keys.into_iter().map(move |key| self.lookup(key))
    }

    pub fn par_map_lookups<'a, I>(&'a self, keys: I) -> impl ParallelIterator<Item=T2> + 'a
    where
        I: ToParallelIterator<T1> + 'a,
    {
        keys.into_par_iter().map(move |key| self.lookup(key))
    }
}
impl_fbitfbit_lookup_table!(f32, f32);
impl_fbitfbit_lookup_table!(f64, f64);
impl_fbitfbit_lookup_table!(f32, f64);
impl_fbitfbit_lookup_table!(f64, f32);


#[derive(Debug, Clone)]
pub struct CyclingFloatLookupTable<T1, T2>
where 
    T1: Float + std::marker::Send + std::marker::Sync,
    T2: Float + std::marker::Send + std::marker::Sync,
    T1: ToIndex,
{
    lookup_table: FloatLookupTable<T1, T2>,
    lower_bound: T1,
    range: T1,
}
impl<T1, T2> CyclingFloatLookupTable<T1, T2>
where
    T1: Float + std::marker::Send + std::marker::Sync,
    T1: ToIndex,
    f64: From<T1>,
    T2: Float + std::marker::Send + std::marker::Sync,
{
    pub fn new(keys: [T1; TABLE_SIZE], values: [T2; TABLE_SIZE], lower_bound: T1, upper_bound: T1) -> Self {
        CyclingFloatLookupTable {
            lookup_table: FloatLookupTable::new(keys, values),
            lower_bound: lower_bound,
            range: upper_bound - lower_bound,
        }
    }

    pub fn lookup(&self, key: T1) -> T2 {
        self.lookup_table.lookup(
            (key % self.range) + self.lower_bound
        )
    }

    pub fn map_lookups<'a, I>(&'a self, keys: I) -> impl Iterator<Item=T2> + 'a
    where
        I: ToIterator<T1> + 'a,
    {
        self.lookup_table.map_lookups(keys)
    }

    pub fn par_map_lookups<'a, I>(&'a self, keys: I) -> impl ParallelIterator<Item=T2> + 'a
    where
        I: ToParallelIterator<T1> + 'a,
    {
        self.lookup_table.par_map_lookups(keys)
    }
}
impl_cycling_fbitfbit_lookup_table!(f32, f32);
impl_cycling_fbitfbit_lookup_table!(f64, f64);
impl_cycling_fbitfbit_lookup_table!(f32, f64);
impl_cycling_fbitfbit_lookup_table!(f64, f32);


#[derive(Debug, Clone)]
pub struct EndoSinLookupTable<T>
where
    T: Float + FloatConst + std::marker::Send + std::marker::Sync,
    T: ToIndex,
{
    lookup_table: CyclingFloatLookupTable<T, T>,
}
impl<T> EndoSinLookupTable<T>
where
    T: Float + FloatConst + std::marker::Send + std::marker::Sync,
    T: ToIndex,
    f64: From<T>,
{
    pub fn key_to_value(&self, key: T) -> T {
        const STEP_SIZE: f64 = f64_consts::FRAC_PI_2 / TABLE_SIZE as f64;
        let lookup_key = (key / (T::from(STEP_SIZE)).unwrap()).trunc().to_usize().unwrap();
        self.lookup_table.lookup_table.values[lookup_key]
    }

    fn direct_lookup(&self, key: T) -> T {
        self.lookup_table.lookup_table.get_next(key)
    }

    pub fn lookup(&self, key: T) -> T {
        let mut abs_key = key.abs();
        let comparisons: [T; 4] = [T::FRAC_PI_2(), T::PI(), T::PI() + T::FRAC_PI_2(), T::TAU()];
        let quadrant: usize =
            comparisons
                .iter()
                .position(|&x: &T| -> bool { abs_key < x })
                .unwrap_or_else(
                    || -> usize {
                        abs_key = abs_key % T::TAU();
                        comparisons
                            .iter()
                            .position(|&x: &T| -> bool { abs_key < x })
                            .expect("This should literally be mathematically impossible")
                    }
                );
        match (key.is_sign_negative(), quadrant) {
            (true,  0) => self.direct_lookup(abs_key).neg(),
            (true,  1) => self.direct_lookup(T::PI() - abs_key).neg(),
            (true,  2) => self.direct_lookup(abs_key - T::PI()),
            (true,  3) => self.direct_lookup(T::TAU() - abs_key),
            (false, 0) => self.direct_lookup(abs_key),
            (false, 1) => self.direct_lookup(T::PI() - abs_key),
            (false, 2) => self.direct_lookup(abs_key - T::PI()).neg(),
            (false, 3) => self.direct_lookup(T::TAU() - abs_key).neg(),
            _ => panic!("Something very strange has happened"),
        }
    }

    pub fn map_lookups<'a, I>(&'a self, keys: I) -> impl Iterator<Item=T> + 'a
    where
        I: ToIterator<T> + 'a,
    {
        keys.into_iter().map(move |key| self.lookup(key))
    }

    pub fn par_map_lookups<'a, I>(&'a self, keys: I) -> impl ParallelIterator<Item=T> + 'a
    where
        I: ToParallelIterator<T> + 'a,
    {
        keys.into_par_iter().map(move |key| self.lookup(key))
    }
}
impl EndoSinLookupTable<f32>
{
    pub const fn new() -> Self {
        EndoSinLookupTable {
            lookup_table: CyclingFloatLookupTable::<f32, f32>::new_const(
                SIN_F32_KEYS, SIN_F32_VALUES, 0.0f32, f32_consts::PI
            ),
        }
    }
}
impl EndoSinLookupTable<f64>
{
    pub const fn new() -> Self {
        EndoSinLookupTable {
            lookup_table: CyclingFloatLookupTable::<f64, f64>::new_const(
                SIN_F64_KEYS, SIN_F64_VALUES, 0.0f64, f64_consts::PI
            ),
        }
    }
}


#[derive(Debug, Clone)]
pub struct EndoCosLookupTable<T>
where
    T: Float + FloatConst + std::marker::Send + std::marker::Sync,
    T: ToIndex,
{
    lookup_table: EndoSinLookupTable<T>,
}
impl<T> EndoCosLookupTable<T>
where
    T: Float + FloatConst + std::marker::Send + std::marker::Sync,
    T: ToIndex,
    f64: From<T>,
{
    pub fn lookup(&self, key: T) -> T
    {
        self.lookup_table.lookup(key + T::FRAC_PI_2())
    }

    pub fn map_lookups<I>(self, keys: I) -> impl Iterator<Item=T>
    where
        I: ToIterator<T>,
    {
        keys.into_iter().map(move |key| self.lookup(key))
    }

    pub fn par_map_lookups<I>(self, keys: I) -> impl ParallelIterator<Item=T>
    where
        I: ToParallelIterator<T>,
    {
        keys.into_par_iter().map(move |key| self.lookup(key))
    }
}
impl EndoCosLookupTable<f32>
{
    pub const fn new() -> Self {
        EndoCosLookupTable {
            lookup_table: EndoSinLookupTable::<f32>{
                lookup_table: CyclingFloatLookupTable::<f32, f32>{
                    lookup_table: FloatLookupTable::<f32, f32>{
                        keys: SIN_F32_KEYS,
                        max_key: f32_consts::FRAC_PI_2,
                        values: SIN_F32_VALUES,
                    },
                    lower_bound: 0.0,
                    range: f32_consts::PI,
                },
            },
        }
    }
}
impl EndoCosLookupTable<f64>
{
    pub const fn new() -> Self {
        EndoCosLookupTable {
            lookup_table: EndoSinLookupTable::<f64>{
                lookup_table: CyclingFloatLookupTable::<f64, f64>{
                    lookup_table: FloatLookupTable::<f64, f64>{
                        keys: SIN_F64_KEYS,
                        max_key: f64_consts::FRAC_PI_2,
                        values: SIN_F64_VALUES,
                    },
                    lower_bound: 0.0,
                    range: f64_consts::PI,
                },
            },
        }
    }
}