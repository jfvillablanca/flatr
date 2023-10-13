use std::fs::File;

use clap::Parser;

/// Flattenize JSON files
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to JSON file
    #[arg(short, long, value_parser = validate_json_file_path)]
    pub file_path: String,
}

fn validate_json_file_path(path: &str) -> Result<(), String> {
    match File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}
