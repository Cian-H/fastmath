use num_traits::sign::Signed;
use num_traits::float::{Float, FloatConst};
use num_traits::NumCast;
use std::ops::{Sub, Rem};
use serde::{Serialize, Deserialize};
// use packed_simd::f64x4;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FloatLookupTable<T1, T2>
where T1: Float,
      T2: Float,
{
    keys: Vec<T1>,
    values: Vec<T2>,
}
impl<T1, T2> FloatLookupTable<T1, T2>
where T1: Float,
      T2: Float,
{
    pub fn new(mut keys: Vec<T1>, mut values: Vec<T2>) -> Self {
        let mut indices: Vec<_> = (0..keys.len()).collect();
        indices.sort_by(|&i, &j| keys[i].partial_cmp(&keys[j]).unwrap());
        for i in 0..keys.len() {
            while i != indices[i] {
                let swap_index = indices[i];
                keys.swap(i, swap_index);
                values.swap(i, swap_index);
                indices.swap(i, swap_index);
            }
        }
        FloatLookupTable { keys, values }
    }

    #[allow(dead_code)]
    pub fn lookup(&self, key: T1) -> T2 {
        match self.keys.binary_search_by(|probe| probe.partial_cmp(&key).unwrap()) {
            Ok(index) => self.values[index],
            Err(index) => {
                let upper_key = &self.keys[index];
                let upper_val = &self.values[index];
                let low_index = index - 1;
                let lower_key = &self.keys[low_index];
                let lower_val = &self.values[low_index];
                // select nearest neighbour
                let diff_upper = (key - *upper_key).abs();
                let diff_lower = (key - *lower_key).abs();
                let mask = diff_lower <= diff_upper;
                (*lower_val * T2::from(mask as u8).expect("Failed to unwrap mask")) +
                (*upper_val * T2::from(!mask as u8).expect("Failed to unwrap !mask"))
            }
        }
    }
}


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CyclingFloatLookupTable<T1, T2>
where T1: Float,
      T2: Float,
{
    lookup_table: FloatLookupTable<T1, T2>,
    lower_bound: T1,
    upper_bound: T1,
    bound_range: T1,
}
impl<T1, T2> CyclingFloatLookupTable<T1, T2>
where T1: Float,
      T2: Float,
{
    pub fn new(keys: Vec<T1>, values: Vec<T2>, lower_bound: T1, upper_bound: T1) -> Self {
        CyclingFloatLookupTable {
            lookup_table: FloatLookupTable::new(keys, values),
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            bound_range: upper_bound - lower_bound,
        }
    }

    pub fn lookup(&self, key: T1) -> T2 {
        let key = (key % self.bound_range) + self.lower_bound;
        self.lookup_table.lookup(key)
    }
}


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
    pub fn new(precision: usize) -> Self {
        let mut keys = Vec::with_capacity(precision);
        let mut values = Vec::with_capacity(precision);
        let upper_bound = T::PI();
        let step = T::FRAC_PI_2() / <T as NumCast>::from(precision).unwrap();
        for i in 0..precision+1 {
            let key = step * <T as NumCast>::from(i).unwrap();
            let value = key.sin();
            keys.push(key);
            values.push(value);
        }
        EndoSinLookupTable {
            lookup_table: CyclingFloatLookupTable::new(keys, values, T::zero(), upper_bound),
        }
    }

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


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct EndoCosLookupTable<T>
where
    T: Float + FloatConst + Signed + Sub<Output = T> + Rem<Output = T> + NumCast + From<u8>,
{
    lookup_table: EndoSinLookupTable<T>,
}
impl<T> EndoCosLookupTable<T>
where
    T: Float + FloatConst + Signed + Sub<Output = T> + Rem<Output = T> + NumCast + From<u8>,
{
    pub fn new(precision: usize) -> Self {
        EndoCosLookupTable {
            lookup_table: EndoSinLookupTable::new(precision),
        }
    }

    #[allow(dead_code)]
    pub fn lookup(&self, key: T) -> T {
        self.lookup_table.lookup(key + T::FRAC_PI_2())
    }
}