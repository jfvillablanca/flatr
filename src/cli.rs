use std::fs::File;

use clap::Parser;
use serde_json::{from_reader, Value};

/// Flattenize JSON files
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to JSON file
    #[arg(short, long, value_parser = validate_json_file_path)]
    pub file: String,
}

fn validate_json_file_path(path: &str) -> Result<String, String> {
    match File::open(path) {
        Ok(file) => match from_reader::<_, Value>(file) {
            Ok(_) => Ok(String::from(path)),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => Err(format!("{}", e)),
    }
}
