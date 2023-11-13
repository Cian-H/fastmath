#!/home/cianh/.cargo/bin/run-cargo-script
// cargo-deps: plotters, num-traits, fastmath={ path = "." }

// A simple script for visualizing plots during development.

extern crate plotters;
extern crate num_traits;
extern crate fastmath;

use std::rc::Rc;
use std::error::Error;
use fastmath::*;
use plotters::prelude::*;
use num_traits::Float;
use plotters::coord::types::RangedCoordf64;

mod exact {
    include!("/home/cianh/Programming/Git_Projects/fastmath/src/tests/accuracy/exact.rs");
}

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

#[derive(Clone)]
enum ValidFloatFunction {
    F32(Rc<dyn Fn(f32) -> f32>),
    F64(Rc<dyn Fn(f64) -> f64>),
}
impl ValidFloatFunction {
    fn new_f32_func(func: impl Fn(f32) -> f32 + 'static) -> Self {
        ValidFloatFunction::F32(Rc::new(func))
    }

    fn new_f64_func(func: impl Fn(f64) -> f64 + 'static) -> Self {
        ValidFloatFunction::F64(Rc::new(func))
    }

    fn plot_function_f32<DB>(
        self: Self,
        chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
        color: &RGBColor,
        label: &str
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        DB: DrawingBackend,
    {
        let function = match self {
            ValidFloatFunction::F32(f) => f.clone(),
            _ => panic!("Function is not f32"),
        };
        let series_color = RGBColor(color.0, color.1, color.2);
        chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f32 / 100.0).map(|x| (x as f64, function(x) as f64)),
            series_color,  // Applying color
        ))
        .unwrap()
        .label(label) // Setting label
        .legend(move |(x,y)| PathElement::new(vec![(x,y), (x + 20,y)], &series_color)); // Drawing a legend element

        Ok(())
    }

    fn plot_function_f64<DB>(
        self: Self,
        chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
        color: &RGBColor,
        label: &str
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        DB: DrawingBackend,
    {
        let function = match self {
            ValidFloatFunction::F64(f) => f.clone(),
            _ => panic!("Function is not f64"),
        };

        let series_color = RGBColor(color.0, color.1, color.2);
        chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64 / 100.0).map(|x| (x, function(x))),
            series_color,  // Applying color
        ))
        .unwrap()
        .label(label) // Setting label
        .legend(move |(x,y)| PathElement::new(vec![(x,y), (x + 20,y)], &series_color)); // Drawing a legend element

        Ok(())
    }
}
impl Default for ValidFloatFunction {
    fn default() -> Self {
        ValidFloatFunction::F32(Rc::new(|x: f32| -> f32 { x }))
    }
}
impl Into<ValidFloatFunction> for Rc<dyn Fn(f32) -> f32> {
    fn into(self) -> ValidFloatFunction {
        ValidFloatFunction::F32(self)
    }
}
impl Into<ValidFloatFunction> for Rc<dyn Fn(f64) -> f64> {
    fn into(self) -> ValidFloatFunction {
        ValidFloatFunction::F64(self)
    }
}

fn plot(functions: Vec<ValidFloatFunction>, yrange: (f64, f64), labels: Vec<String>, output: &String) -> Result<(), Box<dyn Error>> {
    let color_palette: Vec<RGBColor> = (0..functions.len()).map(|i| {
        let hue = (i as f64) / (functions.len() as f64);
        ViridisRGB::get_color(hue)
    }).collect();
    let filename = format!("{}.png", *output);
    let root = BitMapBackend::new(&filename, (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    let (ymin, ymax) = yrange;
    let mut chart = ChartBuilder::on(&root)
        .caption(output, ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5f64..5f64, ymin..ymax)?;

    chart.configure_mesh().draw()?;

    for ((function, color), label) in functions.iter().zip(color_palette.iter()).zip(labels.iter()) {
        match function {
            ValidFloatFunction::F32(_) => function.clone().plot_function_f32(&mut chart, color, &label)?,
            ValidFloatFunction::F64(_) => function.clone().plot_function_f64(&mut chart, color, &label)?,
        }
    }

    chart.configure_series_labels()
                .background_style(&WHITE.mix(0.8))
                .border_style(&BLACK)
                .draw()?;

    root.present()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // pow2
    println!("Plotting pow2");
    plot(
        vec![
            ValidFloatFunction::new_f64_func(exact::f64::pow2),
            ValidFloatFunction::new_f32_func(f32::fast_pow2),
            ValidFloatFunction::new_f64_func(f64::fast_pow2),
        ],
        (0f64, 10f64),
        vec![
            String::from("exact::pow2"),
            String::from("f32::fast_pow2"),
            String::from("f64::fast_pow2"),
        ],
        &String::from("tmp/pow2")
    )?;
    // exp
    println!("Plotting exp");
    plot(
        vec![
            ValidFloatFunction::new_f64_func(exact::f64::exp),
            ValidFloatFunction::new_f32_func(f32::fast_exp),
            ValidFloatFunction::new_f64_func(f64::fast_exp),
        ],
        (0f64, 10f64),
        vec![
            String::from("exact::exp"),
            String::from("f32::fast_exp"),
            String::from("f64::fast_exp"),
        ],
        &String::from("tmp/exp")
    )?;
    // sin
    println!("Plotting sin");
    plot(
        vec![
            ValidFloatFunction::new_f64_func(exact::f64::sin),
            ValidFloatFunction::new_f32_func(f32::fast_sin),
            ValidFloatFunction::new_f64_func(f64::fast_sin),
            ValidFloatFunction::new_f32_func(f32::lookup_sin),
            ValidFloatFunction::new_f64_func(f64::lookup_sin),
        ],
        (-1.5f64, 1.5f64),
        vec![
            String::from("exact::sin"),
            String::from("f32::fast_sin"),
            String::from("f64::fast_sin"),
            String::from("f32::lookup_sin"),
            String::from("f64::lookup_sin"),
        ],
        &String::from("tmp/sin")
    )?;
    // cos
    println!("Plotting cos");
    plot(
        vec![
            ValidFloatFunction::new_f64_func(exact::f64::cos),
            ValidFloatFunction::new_f32_func(f32::fast_cos),
            ValidFloatFunction::new_f64_func(f64::fast_cos),
            ValidFloatFunction::new_f32_func(f32::lookup_cos),
            ValidFloatFunction::new_f64_func(f64::lookup_cos),
        ],
        (-1.5f64, 1.5f64),
        vec![
            String::from("exact::cos"),
            String::from("f32::fast_cos"),
            String::from("f64::fast_cos"),
            String::from("f32::lookup_cos"),
            String::from("f64::lookup_cos"),
        ],
        &String::from("tmp/cos")
    )?;
    // sigmoid
    println!("Plotting sigmoid");
    plot(
        vec![
            ValidFloatFunction::new_f64_func(exact::f64::sigmoid),
            ValidFloatFunction::new_f32_func(f32::fast_sigmoid),
            ValidFloatFunction::new_f64_func(f64::fast_sigmoid),
        ],
        (-1.5f64, 1.5f64),
        vec![
            String::from("exact::sigmoid"),
            String::from("f32::fast_sigmoid"),
            String::from("f64::fast_sigmoid"),
        ],
        &String::from("tmp/sigmoid")
    )?;
    Ok(())
}