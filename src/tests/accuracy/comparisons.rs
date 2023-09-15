#[cfg(test)]

use super::exact;

use num_traits::Float;

fn calculate_percentage_error<T>(vector1: &[T], vector2: &[T]) -> T 
    where T: Float,
{
    let n = vector1.len();
    assert_eq!(n, vector2.len(), "Vectors must have equal lengths.");

    let mut total_error = T::zero();
    for i in 0..n {
        let diff = (vector1[i] - vector2[i]).abs();
        let error = diff / if vector1[i] == T::zero() { T::min_positive_value() } else { vector1[i] };
        total_error = total_error + error;
    }

    let average_error = total_error / T::from(n).unwrap();
    let percentage_error = average_error * T::from(100).expect("Cannot convert 100 to type T");
    percentage_error
}

macro_rules! panic_if_nan_or_print {
    ($x:expr, $varname:expr) => {
        if $x.is_nan() {
            Err(format!("{} is NaN!", $varname))?
        } else {
            println!("{}: {}%", $varname, $x);
            Ok($x)
        }
    }
}

pub mod f64 {
    use crate::*;
    use super::exact;
    use super::calculate_percentage_error;
    
    include!("x.rs");

    pub fn pow2() -> Result<f64, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F64.iter().map(|&x| x.fast_pow2()).collect::<Vec<f64>>(),
            &X_F64.iter().map(|&x| exact::f64::pow2(x)).collect::<Vec<f64>>()
        );
        panic_if_nan_or_print!(percentage_error, "fast_pow2<f64> percentage error")
    }

    pub fn exp() -> Result<f64, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F64.iter().map(|&x| x.fast_exp()).collect::<Vec<f64>>(),
            &X_F64.iter().map(|&x| exact::f64::exp(x)).collect::<Vec<f64>>()
        );
        panic_if_nan_or_print!(percentage_error, "fast_exp<f64> percentage error")
    }

    pub fn cos() -> Result<f64, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F64.iter().map(|&x| x.fast_cos()).collect::<Vec<f64>>(),
            &X_F64.iter().map(|&x| exact::f64::cos(x)).collect::<Vec<f64>>()
        );
        panic_if_nan_or_print!(percentage_error, "fast_cos<f64> percentage error")
    }
    
    pub fn cos_lookup() -> Result<f64, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F64.iter().map(|&x| x.lookup_cos()).collect::<Vec<f64>>(),
            &X_F64.iter().map(|&x| exact::f64::cos(x)).collect::<Vec<f64>>()
        );
        panic_if_nan_or_print!(percentage_error,  "lookup_cos<f64> percentage error")
    }

    pub fn sigmoid() -> Result<f64, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F64.iter().map(|&x| x.fast_sigmoid()).collect::<Vec<f64>>(),
            &X_F64.iter().map(|&x| exact::f64::sigmoid(x)).collect::<Vec<f64>>()
        );
        panic_if_nan_or_print!(percentage_error,  "fast_sigmoid<f64> percentage error")
    }
}

pub mod f32 {
    use crate::*;
    use super::exact;
    use super::calculate_percentage_error;

    include!("x.rs");

    pub fn pow2() -> Result<f32, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F32.iter().map(|&x| x.fast_pow2()).collect::<Vec<f32>>(),
            &X_F32.iter().map(|&x| exact::f32::pow2(x)).collect::<Vec<f32>>()
        );
        panic_if_nan_or_print!(percentage_error, "fast_pow2<f32> percentage error")
    }

    pub fn exp() -> Result<f32, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F32.iter().map(|&x| x.fast_exp()).collect::<Vec<f32>>(),
            &X_F32.iter().map(|&x| exact::f32::exp(x)).collect::<Vec<f32>>()
        );
        panic_if_nan_or_print!(percentage_error, "fast_exp<f32> percentage error")
    }

    pub fn cos() -> Result<f32, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F32.iter().map(|&x| x.fast_cos()).collect::<Vec<f32>>(),
            &X_F32.iter().map(|&x| exact::f32::cos(x)).collect::<Vec<f32>>()
        );
        panic_if_nan_or_print!(percentage_error, "fast_cos<f32> percentage error")
    }
    
    pub fn cos_lookup() -> Result<f32, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F32.iter().map(|&x| x.lookup_cos()).collect::<Vec<f32>>(),
            &X_F32.iter().map(|&x| exact::f32::cos(x)).collect::<Vec<f32>>()
        );
        panic_if_nan_or_print!(percentage_error,  "lookup_cos<f32> percentage error")
    }

    pub fn sigmoid() -> Result<f32, Box<dyn std::error::Error>> {
        let percentage_error = calculate_percentage_error(
            &X_F32.iter().map(|&x| x.fast_sigmoid()).collect::<Vec<f32>>(),
            &X_F32.iter().map(|&x| exact::f32::sigmoid(x)).collect::<Vec<f32>>()
        );
        panic_if_nan_or_print!(percentage_error,  "fast_sigmoid<f32> percentage error")
    }
}