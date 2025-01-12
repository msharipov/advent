use serde_json::Value;

pub fn count_numbers(json: &Value) -> Option<i64> {
    match json {
        Value::Array(vec) => Some(vec.iter().filter_map(|v| count_numbers(v)).sum()),
        Value::Object(map) => Some(map.values().filter_map(|v| count_numbers(v)).sum()),
        Value::Number(num) => num.as_i64(),
        _ => Some(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_numbers_test_1() {
        let string = r#"{}"#;
        let json: Value = serde_json::from_str(string).unwrap();
        assert_eq!(count_numbers(&json), Some(0));
    }

    #[test]
    fn count_numbers_test_2() {
        let string = r#"{
            "name" : "Jimmy",
            "age" : 30,
            "numbers" : [
                1,
                2,
                3,
                4
            ],
            "phones" : {
                "home" : 12345678900,
                "work" : 98765432100
            }
        }"#;
        let json: Value = serde_json::from_str(string).unwrap();
        assert_eq!(count_numbers(&json), Some(111111111040));
    }
}
