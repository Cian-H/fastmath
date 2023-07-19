// build.rs

mod precalculate_lookup_tables {
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    include!("src/lookup/lookup_table.rs");
    use bincode::serialize;

    const PRECISION: usize = 1000;

    fn precalculate_sin_tables() -> Result<(), Box<dyn std::error::Error>> {
        let data = serialize(&EndoSinLookupTable::<f32>::new(PRECISION))?;
        let mut file = File::create("src/lookup/data/sin_f32.bin")?;
        file.write_all(&data)?;

        let data = serialize(&EndoSinLookupTable::<f64>::new(PRECISION))?;
        let mut file = File::create("src/lookup/data/sin_f64.bin")?;
        file.write_all(&data)?;

        Ok(())
    }

    fn precalculate_cos_tables() -> Result<(), Box<dyn std::error::Error>> {
        let data = serialize(&EndoCosLookupTable::<f32>::new(PRECISION))?;
        let mut file = File::create("src/lookup/data/cos_f32.bin")?;
        file.write_all(&data)?;

        let data = serialize(&EndoCosLookupTable::<f64>::new(PRECISION))?;
        let mut file = File::create("src/lookup/data/cos_f64.bin")?;
        file.write_all(&data)?;

        Ok(())
    }

    pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("src/lookup/data")?;
    
        precalculate_sin_tables()?;
        precalculate_cos_tables()?;
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    precalculate_lookup_tables::generate()?;

    Ok(())
}