//tests.rs

use num_traits::Float;

fn calculate_percentage_error<T>(vector1: &[T], vector2: &[T]) -> T 
    where T: Float + std::ops::AddAssign,
{
    let n = vector1.len();
    assert_eq!(n, vector2.len(), "Vectors must have equal lengths.");

    let mut total_error = T::zero();
    for i in 0..n {
        let diff = (vector1[i] - vector2[i]).abs();
        let error = diff / if vector1[i] == T::zero() { T::min_positive_value() } else { vector1[i] };
        total_error += error;
    }

    let average_error = total_error / T::from(n).unwrap();
    let percentage_error = average_error * T::from(100).expect("Cannot convert 100 to type T");
    percentage_error
}

#[cfg(test)]
mod tests {
    mod f64_error {
        use crate::*;
        use super::super::calculate_percentage_error;
        use once_cell::sync::Lazy;

        const TOLERANCE: f64 = 2.5;

        static X: Lazy<Vec<f64>> = Lazy::new(|| {
            (-10000..10000)
            .map(|a| (a as f64) / 1000.)
            .collect::<Vec<f64>>()
        });

        #[test]
        fn pow2() -> Result<(), Box<dyn std::error::Error>> {
            let percentage_error = calculate_percentage_error(
                &X.iter().map(|&x| x.fast_pow2()).collect::<Vec<f64>>(),
                &X.iter().map(|&x| x.powi(2)).collect::<Vec<f64>>()
            );
            assert!(!percentage_error.is_nan(), "fast_pow2<f64> percentage error is NaN");
            assert!(
                percentage_error < TOLERANCE,
                "fast_pow2<f64> percentage error: {0}",
                percentage_error
            );
            Ok(())
        }

        #[test]
        fn exp() -> Result<(), Box<dyn std::error::Error>> {
            let percentage_error = calculate_percentage_error(
                &X.iter().map(|&x| x.fast_exp()).collect::<Vec<f64>>(),
                &X.iter().map(|&x| x.exp()).collect::<Vec<f64>>()
            );
            assert!(!percentage_error.is_nan(), "fast_exp<f64> percentage error is NaN");
            assert!(
                percentage_error < TOLERANCE,
                "fast_exp<f64> percentage error: {0}",
                percentage_error
            );
            Ok(())
        }

        #[test]
        fn cos() -> Result<(), Box<dyn std::error::Error>> {
            let percentage_error = calculate_percentage_error(
                &X.iter().map(|&x| x.fast_cos()).collect::<Vec<f64>>(),
                &X.iter().map(|&x| x.cos()).collect::<Vec<f64>>()
            );
            assert!(!percentage_error.is_nan(), "fast_cos<f64> percentage error is NaN");
            assert!(
                percentage_error < TOLERANCE,
                "fast_cos<f64> percentage error: {0}",
                percentage_error
            );
            Ok(())
        }

        #[test]
        fn sigmoid() -> Result<(), Box<dyn std::error::Error>> {
            let percentage_error = calculate_percentage_error(
                &X.iter().map(|&x| x.fast_sigmoid()).collect::<Vec<f64>>(),
                &X.iter().map(|&x| sigmoid_builtin_f64(x)).collect::<Vec<f64>>()
            );
            assert!(!percentage_error.is_nan(), "fast_sigmoid<f64> percentage error is NaN");
            assert!(
                percentage_error < TOLERANCE,
                "fast_sigmoid<f64> percentage error: {0}",
                percentage_error
            );
            Ok(())
        }
    }

    mod f32_error {
        use crate::*;
        use super::super::calculate_percentage_error;
        use once_cell::sync::Lazy;

        const TOLERANCE: f32 = 2.5;

        static X: Lazy<Vec<f32>> = Lazy::new(|| {
            (-10000..10000)
            .map(|a| (a as f32) / 1000.)
            .collect::<Vec<f32>>()
        });

        #[test]
        fn pow2() -> Result<(), Box<dyn std::error::Error>> {
            assert!(
                calculate_percentage_error(
                    &X.iter().map(|&x| x.fast_pow2()).collect::<Vec<f32>>(),
                    &X.iter().map(|&x| x.powi(2)).collect::<Vec<f32>>()
                ) < TOLERANCE
            );
            Ok(())
        }

        #[test]
        fn exp() -> Result<(), Box<dyn std::error::Error>> {
            assert!(
                calculate_percentage_error(
                    &X.iter().map(|&x| x.fast_exp()).collect::<Vec<f32>>(),
                    &X.iter().map(|&x| x.exp()).collect::<Vec<f32>>()
                ) < TOLERANCE
            );
            Ok(())
        }

        #[test]
        fn cos() -> Result<(), Box<dyn std::error::Error>> {
            assert!(
                calculate_percentage_error(
                    &X.iter().map(|&x| x.fast_cos()).collect::<Vec<f32>>(),
                    &X.iter().map(|&x| x.cos()).collect::<Vec<f32>>()
                ) < TOLERANCE
            );
            Ok(())
        }

        #[test]
        fn sigmoid() -> Result<(), Box<dyn std::error::Error>> {
            assert!(
                calculate_percentage_error(
                    &X.iter().map(|&x| x.fast_sigmoid()).collect::<Vec<f32>>(),
                    &X.iter().map(|&x| sigmoid_builtin_f32(x)).collect::<Vec<f32>>()
                ) < TOLERANCE
            );
            Ok(())
        }
    }
}