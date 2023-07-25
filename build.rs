// build.rs

mod precalculate_lookup_tables {
    use std::f32::consts as f32_consts;
    use std::f64::consts as f64_consts;
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    include!("src/lookup/config.rs");
    // use bincode::serialize;

    // fn precalculate_sin_tables() -> Result<(), Box<dyn std::error::Error>> {
    //     let data = serialize(&EndoSinLookupTable::<f32>::new(PRECISION))?;
    //     let mut file = File::create("src/lookup/data/sin_f32.bin")?;
    //     file.write_all(&data)?;

    //     let data = serialize(&EndoSinLookupTable::<f64>::new(PRECISION))?;
    //     let mut file = File::create("src/lookup/data/sin_f64.bin")?;
    //     file.write_all(&data)?;

    //     Ok(())
    // }

    // fn precalculate_cos_tables() -> Result<(), Box<dyn std::error::Error>> {
    //     let data = serialize(&EndoCosLookupTable::<f32>::new(PRECISION))?;
    //     let mut file = File::create("src/lookup/data/cos_f32.bin")?;
    //     file.write_all(&data)?;

    //     let data = serialize(&EndoCosLookupTable::<f64>::new(PRECISION))?;
    //     let mut file = File::create("src/lookup/data/cos_f64.bin")?;
    //     file.write_all(&data)?;

    //     Ok(())
    // }

    // fn precalculate_sin_tables() -> Result<(), Box<dyn std::error::Error>> {
    //     let step: f32 = f32_consts::FRAC_PI_2 / TABLE_SIZE as f32;
    //     let half_step: f32 = step / 2.0;

    //     let keys: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f32)) - half_step
    //     }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
    //     let values: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f32)).sin()
    //     }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
    //     let data = format!("pub const KEYS: [f32; {}] = {:?};\npub const VALUES: [f32; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

    //     let mut file = File::create("src/lookup/data/sin_f32.rs")?;
    //     file.write_all(data.as_bytes())?;

    //     let step: f64 = f64_consts::FRAC_PI_2 / TABLE_SIZE as f64;
    //     let half_step: f64 = step / 2.0;

    //     let keys: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f64)) - half_step
    //     }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
    //     let values: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f64)).sin()
    //     }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
    //     let data = format!("pub const KEYS: [f64; {}] = {:?};\npub const VALUES: [f64; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

    //     let mut file = File::create("src/lookup/data/sin_f64.rs")?;
    //     file.write_all(data.as_bytes())?;

    //     Ok(())
    // }

    macro_rules! precalculate_sin_tables {
        () => {{
            let step: f32 = f32_consts::FRAC_PI_2 / TABLE_SIZE as f32;
            let half_step: f32 = step / 2.0;

            let keys: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                (step * (i as f32)) - half_step
            }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
            let values: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                (step * (i as f32)).sin()
            }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
            let data = format!("pub const SIN_F32_KEYS: [f32; {}] = {:?};\npub const SIN_F32_VALUES: [f32; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

            let mut file = File::create("src/lookup/data/sin_f32.rs").expect("Failed to create sin_f32.rs");
            file.write_all(data.as_bytes()).expect("Failed to write sin_f32.rs");

            let step: f64 = f64_consts::FRAC_PI_2 / TABLE_SIZE as f64;
            let half_step: f64 = step / 2.0;

            let keys: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                (step * (i as f64)) - half_step
            }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
            let values: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
                (step * (i as f64)).sin()
            }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
            let data = format!("pub const SIN_F64_KEYS: [f64; {}] = {:?};\npub const SIN_F64_VALUES: [f64; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

            let mut file = File::create("src/lookup/data/sin_f64.rs").expect("Failed to create sin_f64.rs");
            file.write_all(data.as_bytes()).expect("Failed to write sin_f64.rs");
        }};
    }
    // fn precalculate_sin_tables() -> Result<(), Box<dyn std::error::Error>> {
    //     let step: f32 = f32_consts::FRAC_PI_2 / TABLE_SIZE as f32;
    //     let half_step: f32 = step / 2.0;

    //     let keys: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f32)) - half_step
    //     }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
    //     let values: [f32; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f32)).sin()
    //     }).collect::<Vec<f32>>().try_into().unwrap_or([0.0f32; TABLE_SIZE]);
    //     let data = format!("pub const KEYS: [f32; {}] = {:?};\npub const VALUES: [f32; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

    //     let mut file = File::create("src/lookup/data/sin_f32.rs")?;
    //     file.write_all(data.as_bytes())?;

    //     let step: f64 = f64_consts::FRAC_PI_2 / TABLE_SIZE as f64;
    //     let half_step: f64 = step / 2.0;

    //     let keys: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f64)) - half_step
    //     }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
    //     let values: [f64; TABLE_SIZE] = (0..TABLE_SIZE).map(|i| {
    //         (step * (i as f64)).sin()
    //     }).collect::<Vec<f64>>().try_into().unwrap_or([0.0f64; TABLE_SIZE]);
    //     let data = format!("pub const KEYS: [f64; {}] = {:?};\npub const VALUES: [f64; {}] = {:?};\n", TABLE_SIZE, keys, TABLE_SIZE, values);

    //     let mut file = File::create("src/lookup/data/sin_f64.rs")?;
    //     file.write_all(data.as_bytes())?;

    //     Ok(())
    // }

    pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("src/lookup/data")?;
    
        precalculate_sin_tables!();
        // precalculate_cos_tables()?;
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    precalculate_lookup_tables::generate()?;

    Ok(())
}