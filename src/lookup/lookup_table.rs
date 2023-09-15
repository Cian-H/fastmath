use std::f32::consts as f32_consts;
use std::f64::consts as f64_consts;
use std::cmp::Ordering;
use num_traits::identities::One;
use num_traits::ops::wrapping::WrappingSub;
use num_traits::float::{Float, FloatConst};

use crate::{
    impl_fbitfbit_lookup_table,
    impl_cycling_fbitfbit_lookup_table,
};
use crate::lookup::TABLE_SIZE;
use crate::lookup::ordinal_float::FloatOrd;
use crate::lookup::const_tables::*;


#[derive(Debug, Clone)]
pub struct FloatLookupTable<T1, T2>
where 
    T1: Float,
    T2: Float,
    FloatOrd<T1>: Ord,
{
    keys: [FloatOrd<T1>; TABLE_SIZE],
    values: [T2; TABLE_SIZE],
}
impl<T1, T2> FloatLookupTable<T1, T2>
where 
    T1: Float,
    T2: Float,
    FloatOrd<T1>: Ord,
{
    pub fn new(keys: [T1; TABLE_SIZE], values: [T2; TABLE_SIZE]) -> Self {
        FloatLookupTable {
            keys: keys.map(|key| FloatOrd(key)),
            values,
        }
    }

    pub fn get_next(&self, key: T1) -> T2
    {
        let ord_key = FloatOrd(key);
        let mut lower_bound = 0;
        let mut upper_bound = self.keys.len() - 1;
        let mut mid: usize;

        while lower_bound < upper_bound {
            mid = lower_bound + (upper_bound - lower_bound) / 2;
            if self.keys[mid] < ord_key {
                lower_bound = mid + 1;
            } else {
                upper_bound = mid;
            }
        }
        self.values[upper_bound]
    }

    pub fn lookup(&self, key: T1) -> T2 {
        self.get_next(key)
    }
}
impl_fbitfbit_lookup_table!(f32, f32);
impl_fbitfbit_lookup_table!(f64, f64);
impl_fbitfbit_lookup_table!(f32, f64);
impl_fbitfbit_lookup_table!(f64, f32);


#[derive(Debug, Clone)]
pub struct CyclingFloatLookupTable<T1, T2>
where 
    T1: Float,
    T2: Float,
    FloatOrd<T1>: Ord,
{
    lookup_table: FloatLookupTable<T1, T2>,
    lower_bound: T1,
    range: T1,
}
impl<T1, T2> CyclingFloatLookupTable<T1, T2>
where
    T1: Float,
    T2: Float,
    FloatOrd<T1>: Ord,
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
}
impl_cycling_fbitfbit_lookup_table!(f32, f32);
impl_cycling_fbitfbit_lookup_table!(f64, f64);
impl_cycling_fbitfbit_lookup_table!(f32, f64);
impl_cycling_fbitfbit_lookup_table!(f64, f32);


#[derive(Debug, Clone)]
pub struct EndoSinLookupTable<T>
where
    T: Float + FloatConst,
    FloatOrd<T>: Ord,
{
    lookup_table: CyclingFloatLookupTable<T, T>,
}
impl<T> EndoSinLookupTable<T>
where
    T: Float + FloatConst,
    FloatOrd<T>: Ord,
{
    pub fn lookup(&self, key: T) -> T {
        if key < T::zero() {
            -self.lookup(-key)
        } else if key < T::FRAC_PI_2() {
            self.lookup_table.lookup(key)
        } else if key < T::PI() {
            self.lookup_table.lookup(T::PI() - key)
        } else if key < T::TAU() { // obviously, mod is slow so we want to avoid it until this would start recursing deeply
            -self.lookup(key - T::PI())
        } else {
            -self.lookup(key % T::PI())
        }
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
    T: Float + FloatConst,
{
    lookup_table: EndoSinLookupTable<T>,
}
impl<T> EndoCosLookupTable<T>
where
    T: Float + FloatConst,
{
    pub fn lookup(&self, key: T) -> T {
        self.lookup_table.lookup(key + T::FRAC_PI_2())
    }
}
impl EndoCosLookupTable<f32>
{
    pub const fn new() -> Self {
        EndoCosLookupTable {
            lookup_table: EndoSinLookupTable::<f32>::new(),
        }
    }
}
impl EndoCosLookupTable<f64>
{
    pub const fn new() -> Self {
        EndoCosLookupTable {
            lookup_table: EndoSinLookupTable::<f64>::new(),
        }
    }
}