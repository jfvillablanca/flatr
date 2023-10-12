use std::{error::Error, fs::File};

use serde_json::{from_reader, Value};

const INPUT_FILENAME: &str = "test-file.json";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(INPUT_FILENAME)?;
    let json_schema: Value = from_reader(file)?;

    let flattened_strings = flatten_json(&json_schema);
    println!("{:#?}", flattened_strings);
    Ok(())
}

fn flatten_json(input_json: &Value) -> Vec<String> {
    let mut flattened_strings: Vec<String> = vec![];

    {
        let prefix = String::new();
        let flattened_strings: &mut Vec<String> = &mut flattened_strings;
        match input_json {
            Value::Object(obj) => {
                for key in obj.keys() {
                    let new_prefix = format!("{}.{}", prefix, key);
                    flattenizer(&obj[key], new_prefix, flattened_strings);
                }
            }
            Value::Array(array) => {
                for (index, value) in array.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, index);
                    flattenizer(value, new_prefix, flattened_strings);
                }
            }
            Value::String(val) => {
                let entry = format!("{} = \"{}\"", prefix, val);
                flattened_strings.push(entry);
            }
            Value::Number(val) => {
                let entry = format!("{} = {}", prefix, val);
                flattened_strings.push(entry);
            }
            Value::Bool(val) => {
                let entry = format!("{} = {}", prefix, val);
                flattened_strings.push(entry);
            }
            Value::Null => {
                let entry = format!("{} = null", prefix);
                flattened_strings.push(entry);
            }
        }
    };

    flattened_strings
}

fn flattenizer(input_json: &Value, prefix: String, flattened_strings: &mut Vec<String>) {
    match input_json {
        Value::Object(obj) => {
            for key in obj.keys() {
                let new_prefix = format!("{}.{}", prefix, key);
                flattenizer(&obj[key], new_prefix, flattened_strings);
            }
        }
        Value::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, index);
                flattenizer(value, new_prefix, flattened_strings);
            }
        }
        Value::String(val) => {
            let entry = format!("{} = \"{}\"", prefix, val);
            flattened_strings.push(entry);
        }
        Value::Number(val) => {
            let entry = format!("{} = {}", prefix, val);
            flattened_strings.push(entry);
        }
        Value::Bool(val) => {
            let entry = format!("{} = {}", prefix, val);
            flattened_strings.push(entry);
        }
        Value::Null => {
            let entry = format!("{} = null", prefix);
            flattened_strings.push(entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flatten_json;

    #[test]
    fn test_flatten_json() {
        use serde_json::json;

        let input_json = json!({
            "string": "Hello, world!",
            "number": 42,
            "boolean": true,
            "null_value": null,
            "array": [1, 2, 3],
            "object": {
                "nested_string": "Nested string",
                "nested_number": 123,
                "nested_boolean": false,
                "nested_null_value": null,
                "nested_array": ["a", "b", "c"],
                "nested_object": {
                    "more_nested_string": "More nested string",
                    "more_nested_number": 456,
                    "more_nested_boolean": true,
                    "more_nested_null_value": null,
                    "more_nested_array": ["x", "y", "z"]
                }
            }
        });

        let expected_result = vec![
            ".string = \"Hello, world!\"",
            ".number = 42",
            ".boolean = true",
            ".null_value = null",
            ".array[0] = 1",
            ".array[1] = 2",
            ".array[2] = 3",
            ".object.nested_string = \"Nested string\"",
            ".object.nested_number = 123",
            ".object.nested_boolean = false",
            ".object.nested_null_value = null",
            ".object.nested_array[0] = \"a\"",
            ".object.nested_array[1] = \"b\"",
            ".object.nested_array[2] = \"c\"",
            ".object.nested_object.more_nested_string = \"More nested string\"",
            ".object.nested_object.more_nested_number = 456",
            ".object.nested_object.more_nested_boolean = true",
            ".object.nested_object.more_nested_null_value = null",
            ".object.nested_object.more_nested_array[0] = \"x\"",
            ".object.nested_object.more_nested_array[1] = \"y\"",
            ".object.nested_object.more_nested_array[2] = \"z\"",
        ];

        let actual_result = flatten_json(&input_json);

        assert_eq!(actual_result, expected_result);
    }
}
