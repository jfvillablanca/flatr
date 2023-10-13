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
// Integration test
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_validate_json_file_path() {
        // Create a temporary JSON file for testing
        let test_file_name = "test.json";
        let json_data = r#"{"key": "value"}"#;
        let mut file = File::create(&test_file_name).expect("Unable to create test file");

        file.write_all(json_data.as_bytes())
            .expect("Unable to write to test file");

        // Use the function to validate the JSON file path
        assert_eq!(validate_json_file_path(&test_file_name), Ok(()));

        // Clean up the test file
        std::fs::remove_file(&test_file_name).expect("Unable to remove test file");
    }
}
