// build.rs

mod precalculate_lookup_tables {
    use std::f32::consts as f32_consts;
    use std::f64::consts as f64_consts;
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    include!("src/lookup/config.rs");
    include!("src/lookup/ordinal_float.rs");

    macro_rules! precalculate_sin_tables {
        () => {{
            let step: f32 = f32_consts::FRAC_PI_2 / TABLE_SIZE as f32;
            let half_step: f32 = step / 2.0;

            let keys: [FloatOrd<f32>; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                FloatOrd( (step * (i as f32)) - half_step )
            }).collect::<Vec<FloatOrd<f32>>>().try_into().unwrap_or([FloatOrd::new(); TABLE_SIZE]);
            let values: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                (step * (i as f32)).sin()
            }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
            let data = format!("pub(crate) const SIN_F32_KEYS: [FloatOrd<f32>; {}] = {:?};\npub const SIN_F32_VALUES: [f32; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

            let mut file = File::create("src/lookup/data/sin_f32.rs")?;
            file.write_all(data.as_bytes())?;

            let step: f64 = f64_consts::FRAC_PI_2 / TABLE_SIZE as f64;
            let half_step: f64 = step / 2.0;

            let keys: [FloatOrd<f64>; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                FloatOrd( (step * (i as f64)) - half_step )
            }).collect::<Vec<FloatOrd<f64>>>().try_into().unwrap_or([FloatOrd::new(); TABLE_SIZE]);
            let values: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                (step * (i as f64)).sin()
            }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
            let data = format!("pub const SIN_F64_KEYS: [FloatOrd<f64>; {}] = {:?};\npub const SIN_F64_VALUES: [f64; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

            let mut file = File::create("src/lookup/data/sin_f64.rs")?;
            file.write_all(data.as_bytes())?;
        }};
    }

    pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("src/lookup/data")?;
    
        precalculate_sin_tables!();
        // precalculate_cos_tables()?;
        
        Ok(())
    }
}

mod precalculate_test_tables {
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    include!("src/tests/accuracy/config.rs");

    macro_rules! precalculate_test_tables {
        () => {{
            let scaling: f32 = X_SIZE as f32 / (X_MAX as f32 - X_MIN as f32);
            let x_f32: [f32; X_SIZE] = 
                            (0..X_SIZE)
                            .map(|a| ((a as f32) / scaling) + X_MIN as f32)
                            .collect::<Vec<f32>>()
                            .try_into().map_err(|_| "Failed to convert Vec<f32> to [f32; X_SIZE]")?;

            let scaling: f64 = X_SIZE as f64 / (X_MAX as f64 - X_MIN as f64);
            let x_f64: [f64; X_SIZE] = 
                            (0..X_SIZE)
                            .map(|a| ((a as f64) / scaling) + X_MIN as f64)
                            .collect::<Vec<f64>>()
                            .try_into().map_err(|_| "Failed to convert Vec<f64> to [f64; X_SIZE]")?;

            let data = format!(
                "#[cfg(test)]\n#[allow(dead_code)]\npub const X_F32: [f32; {}] = {:?};\n#[cfg(test)]\n#[allow(dead_code)]\npub const X_F64: [f64; {}] = {:?};",
                X_SIZE, x_f32, X_SIZE, x_f64
            );

            let mut file = File::create("src/tests/accuracy/x.rs")?;
            file.write_all(data.as_bytes())?;
        }};
    }

    pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("src/tests/accuracy")?;
    
        precalculate_test_tables!();
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    precalculate_lookup_tables::generate()?;
    precalculate_test_tables::generate()?;

    Ok(())
}