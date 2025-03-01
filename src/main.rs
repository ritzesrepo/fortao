use std::env;
use std::fs::File;
use std::io::Read;
use rand::seq::SliceRandom;
use serde::Deserialize;

// Define struct to hold the fortaos from the TOML files
#[derive(Deserialize)]
struct Fortaos {
    fortaos: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [<file> ...]", args[0]);
        std::process::exit(1);
    }

    // Vector to hold all fortaos from the TOML files
    let mut fortao_vec = Vec::new();

    // Process each TOML file passed
    for filename in &args[1..] {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the TOML file as a Fortaos struct
        let parsed_fortaos: Fortaos = toml::from_str(&contents)?;
        fortao_vec.extend(parsed_fortaos.fortaos);
    }

    if fortao_vec.is_empty() {
        eprintln!("No fortaos found in the provided files.");
        std::process::exit(1);
    }

    // Randomly select and print one of the fortaos
    let mut rng = rand::thread_rng();
    if let Some(fortao) = fortao_vec.choose(&mut rng) {
        println!("{}", fortao);
    }

    Ok(())
}
