use std::f32::consts as f32_consts;
use std::f64::consts as f64_consts;
use num_traits::float::{Float, FloatConst};
use crate::lookup::TABLE_SIZE;
use crate::lookup::const_tables::*;

// This function should never be used in a non-const context.
// It only exists as a workaround for the fact that const fn's cannot use iterators.
const fn make_ordinal<T: Float>(
    input: [T; TABLE_SIZE],
    mut map_target: [FloatOrd<T>; TABLE_SIZE],
) -> () {
    // let mut map_target = [FloatOrd::<T>::new(); TABLE_SIZE];
    let mut index = 0;
    while index < TABLE_SIZE {
        map_target[index] = FloatOrd(input[index]);
        index += 1;
    }
}


// The following macros are to minimise the amount of boilerplate for static types on the lookup tables.
macro_rules! impl_fbitfbit_lookup_table {
    ($key_type:ty, $value_type:ty) => {
        impl FloatLookupTable<$key_type, $value_type> {
            pub const fn new_const(keys: [$key_type; TABLE_SIZE], values: [$value_type; TABLE_SIZE]) -> Self {
                let ord_keys: [FloatOrd<$key_type>; TABLE_SIZE] = [FloatOrd(0.0 as $key_type); TABLE_SIZE];
                make_ordinal(keys, ord_keys);
                FloatLookupTable {
                    keys: ord_keys,
                    values,
                }
            }
        }
    };
}

macro_rules! impl_cycling_fbitfbit_lookup_table {
    ($key_type:ty, $value_type:ty) => {
        impl CyclingFloatLookupTable<$key_type, $value_type> {
            pub const fn new_const(keys: [$key_type; TABLE_SIZE], values: [$value_type; TABLE_SIZE], lower_bound: $key_type, upper_bound: $key_type) -> Self {
                CyclingFloatLookupTable {
                    lookup_table: FloatLookupTable::<$key_type, $value_type>::new_const(keys, values),
                    lower_bound,
                    upper_bound,
                }
            }
        }
    };
}


#[derive(Default, Debug,Clone, Copy, PartialEq,  PartialOrd)]
pub struct FloatOrd<T: Float>(pub T);
impl<T: Float> FloatOrd<T> {
    pub fn new() -> Self {
        FloatOrd(T::zero())
    }
}
impl<T: Float> Eq for FloatOrd<T> {}
impl<T: Float> Ord for FloatOrd<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}


#[derive(Debug, Clone)]
pub struct FloatLookupTable<T1, T2>
where 
    T1: Float,
    T2: Float,
{
    keys: [FloatOrd<T1>; TABLE_SIZE],
    values: [T2; TABLE_SIZE],
}
impl<T1, T2> FloatLookupTable<T1, T2>
where 
    T1: Float,
    T2: Float,
{
    pub fn new(keys: [T1; TABLE_SIZE], values: [T2; TABLE_SIZE]) -> Self {
        FloatLookupTable {
            keys: keys.map(|key| FloatOrd(key)),
            values,
        }
    }

    pub fn get_next(&self, key: T1) -> T2 {
        let ord_key = FloatOrd(key);
        let mut lower_bound = 0;
        let mut upper_bound = self.keys.len() - 1;
        let mut mid = (lower_bound + upper_bound) / 2;
        while upper_bound - lower_bound > 1 {
            if self.keys[mid] < ord_key {
                lower_bound = mid;
            } else {
                upper_bound = mid;
            }
            mid = (lower_bound + upper_bound) / 2;
        }
        self.values[mid]
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
{
    lookup_table: FloatLookupTable<T1, T2>,
    lower_bound: T1,
    upper_bound: T1,
}
impl<T1, T2> CyclingFloatLookupTable<T1, T2>
where
    T1: Float,
    T2: Float,
{
    pub fn new(keys: [T1; TABLE_SIZE], values: [T2; TABLE_SIZE], lower_bound: T1, upper_bound: T1) -> Self {
        CyclingFloatLookupTable {
            lookup_table: FloatLookupTable::new(keys, values),
            lower_bound,
            upper_bound,
        }
    }

    pub fn lookup(&self, key: T1) -> T2 {
        let key = (key % (self.upper_bound - self.lower_bound)) + self.lower_bound;
        self.lookup_table.lookup(key)
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
{
    lookup_table: CyclingFloatLookupTable<T, T>,
}
impl<T> EndoSinLookupTable<T>
where
    T: Float + FloatConst,
{
    #[allow(dead_code)]
    pub fn lookup(&self, key: T) -> T {
        if key < T::zero() {
            -self.lookup(-key)
        } else if key < T::FRAC_PI_2() {
            self.lookup_table.lookup(key)
        } else if key < T::PI() {
            self.lookup_table.lookup(T::PI() - key)
        } else {
            -self.lookup(key - T::PI())
        }
    }
}
impl EndoSinLookupTable<f32>
{
    pub const fn new() -> Self {
        const UPPER_BOUND: f32 = f32_consts::PI;

        EndoSinLookupTable {
            lookup_table: CyclingFloatLookupTable::<f32, f32>::new_const(SIN_F32_KEYS, SIN_F32_VALUES, 0.0f32, UPPER_BOUND),
        }
    }
}
impl EndoSinLookupTable<f64>
{
    pub const fn new() -> Self {
        let upper_bound = f64_consts::PI;
        
        EndoSinLookupTable {
            lookup_table: CyclingFloatLookupTable::<f64, f64>::new_const(SIN_F64_KEYS, SIN_F64_VALUES, 0.0f64, upper_bound),
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
    T: Float + FloatConst + std::fmt::Debug,
{
    #[allow(dead_code)]
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