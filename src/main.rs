mod cli;

use std::{error::Error, fs::File};

use clap::Parser;
use flatr::flatten_json;
use serde_json::{from_reader, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    let file = File::open(args.file)?;
    let json_schema: Value = from_reader(file)?;

    flatten_json(&json_schema);
    Ok(())
}
