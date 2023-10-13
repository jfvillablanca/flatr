use serde_json::Value;

pub fn flatten_json(input_json: &Value) -> Vec<String> {
    let mut flattened_strings: Vec<String> = vec![];

    flattenizer(input_json, String::new(), &mut flattened_strings);

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
        _ => {
            let formatted_value = match input_json {
                Value::String(v) => format!("\"{v}\""),
                Value::Number(v) => v.to_string(),
                Value::Bool(v) => v.to_string(),
                Value::Null => "null".to_string(),
                _ => "".to_string(),
            };
            let entry = format!("{} = {}", prefix, formatted_value);
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
