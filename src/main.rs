use csv::ReaderBuilder;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {

    let mut rng = thread_rng();
    let fortao_paths: Vec<PathBuf> = std::env::args_os()
        .skip(1)
        .map(PathBuf::from)
        .collect();
    let mut fortao_vec = Vec::new();

    for file_path in fortao_paths {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            // .quoting(true)
            // .flexible(true)
            .from_path(&file_path)?;
        for result in reader.records() {
            let record = result?;
            fortao_vec.extend(record.iter().map(|field| field.to_string()));
        }
    }

    if let Some(random_choice) = fortao_vec.choose(&mut rng) {
        println!("{}", random_choice);
    }

    Ok(())
}
