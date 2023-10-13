use clap::Parser;

/// Flattenize JSON files
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to JSON file
    #[arg(short, long)]
    pub file_path: String,
}
