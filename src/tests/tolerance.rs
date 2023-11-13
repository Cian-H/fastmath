use super::accuracy;
use serde_json;

fn get_tolerance<T>(key: &str) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned
{
    let json: serde_json::Value = 
        serde_json::from_str(
            include_str!("tolerance.json")
        )?;
    let value = serde_json::value::from_value(
        json[key].clone()
    )?;
    Ok(value)
}

macro_rules! test_within_tolerance {
    ($function:ident, $t:ty, $test_name:ident) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn std::error::Error>> {
            let tolerance: $t = get_tolerance::<$t>(stringify!($test_name)).unwrap_or_else(|_| panic!("Invalid tolerance for {}", stringify!($test_name)));
            let percentage_error: $t = $function()?;
            assert!(percentage_error < tolerance);
            Ok(())
        }
    };
}

mod f64 {
    use super::{accuracy, get_tolerance};
    use accuracy::f64::*;

    test_within_tolerance!(pow2, f64, pow2_fast);
    test_within_tolerance!(exp, f64, exp_fast);
    test_within_tolerance!(cos, f64, cos_fast);
    test_within_tolerance!(cos_lookup, f64, cos_lk);
    test_within_tolerance!(sin, f64, sin_fast);
    test_within_tolerance!(sin_lookup, f64, sin_lk);
    test_within_tolerance!(tan, f64, tan_fast);
    test_within_tolerance!(sigmoid, f64, sigmoid_fast);
}

mod f32 {
    use super::{accuracy, get_tolerance};
    use accuracy::f32::*;

    test_within_tolerance!(pow2, f32, pow2_fast);
    test_within_tolerance!(exp, f32, exp_fast);
    test_within_tolerance!(cos, f32, cos_fast);
    test_within_tolerance!(cos_lookup, f32, cos_lk);
    test_within_tolerance!(sin, f32, sin_fast);
    test_within_tolerance!(sin_lookup, f32, sin_lk);
    test_within_tolerance!(tan, f32, tan_fast);
    test_within_tolerance!(sigmoid, f32, sigmoid_fast);
}