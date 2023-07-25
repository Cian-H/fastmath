use fastmath::LookupCos;

fn main() {
    let x = (-10000..10000)
        .map(|a| (a as f64) / 1000.)
        .collect::<Vec<f64>>();

    let y = x.iter().map(|&x| x.lookup_cos()).collect::<Vec<f64>>();
    
    // to ensure the compiler doesn't optimize away the function call, we print the result
    println!("{:?}", y);
}