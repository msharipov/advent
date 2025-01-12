use serde_json::Value;

fn count_numbers(json: &Value) -> i64 {
    match json {
        Value::Array(vec) => vec.iter().map(|v| count_numbers(v)).sum(),
        Value::Object(map) => map.values().map(|v| count_numbers(v)).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_numbers_test_1() {
        let string = r#"{}"#;
        let json: Value = serde_json::from_str(string).unwrap();
        assert_eq!(count_numbers(&json), 0);
    }
}
