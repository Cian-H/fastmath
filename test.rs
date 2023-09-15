use std::fs::read;
use std::f64::consts as f64_consts;
use bincode::deserialize;
use once_cell::sync::Lazy;
use ndarray::prelude::*;
use optimize::*;
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


fn fast_exp(x: f64, clipp_thresh: f64, v_scale: f64, clipp_shift: f64) -> f64 {
    // const CLIPP_THRESH: f64 = -180335.51911105003;
    // const V_SCALE: f64 = 4524653012949098.0;
    // const CLIPP_SHIFT: f64 = 1018.1563534409383;
    let scaled_p = f64_consts::LOG2_E * x;
    let clipp = scaled_p.max(clipp_thresh);
    let v = (v_scale * (clipp + clipp_shift)) as u64;
    f64::from_bits(v)
}

const Y: Lazy<Vec<f64>> = Lazy::new(|| { deserialize(&read("tmp/Y.bin").unwrap()).unwrap() } );

fn objective(args: ArrayView1<f64>) -> f64 {
    let clipp_thresh: f64 = args[0];
    let v_scale: f64 = args[1];
    let clipp_shift: f64 = args[2];

    let X: Vec<f64> = (-10000..10000)
        .map(|a| (a as f64) / 1000.)
        .collect::<Vec<f64>>();
    let Y_hat: Vec<f64> = X.iter().map(|&x| fast_exp(x, clipp_thresh, v_scale, clipp_shift)).collect::<Vec<f64>>();
    calculate_percentage_error(&(*Y), &Y_hat)
}

fn optimize_params() {
    // Create a minimizer using the builder pattern.
    let minimizer = NelderMeadBuilder::default()
    .xtol(1e-6f64)
    .ftol(1e-6f64)
    .maxiter(50000)
    .build()
    .unwrap();

    // Set the starting guess
    let args: Array1<f64> = Array1::from_vec(vec![-180335.51911105003, 4524653012949098.0, 1018.1563534409383]);

    // Run the optimization
    let ans = minimizer.minimize(objective, args.view());

    // Print the optimized values
    println!("Final optimized arguments: {}", ans);
}