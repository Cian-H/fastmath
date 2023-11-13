use fastmath::LookupCos;
use std::hint::black_box;

fn main() {
    let x = (-10000000..10000000)
        .map(|a| (a as f64) / 1000000.)
        .collect::<Vec<f64>>();

    let y = x.iter().map(|&x| black_box(x).lookup_cos()).collect::<Vec<f64>>();
    
    // to ensure the compiler doesn't optimize away the function call, we print the result
    black_box(y);
    println!("DONE!");
}