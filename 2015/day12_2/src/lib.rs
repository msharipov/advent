use serde_json::Value;

pub fn count_numbers(json: &Value) -> Option<i64> {
    match json {
        Value::Array(vec) => Some(vec.iter().filter_map(|v| count_numbers(v)).sum()),
        Value::Object(map) => {
            if has_red(json) {
                Some(0)
            } else {
                Some(map.values().filter_map(|v| count_numbers(v)).sum())
            }
        }
        Value::Number(num) => num.as_i64(),
        _ => Some(0),
    }
}

fn has_red(json: &Value) -> bool {
    if let Value::Object(map) = json {
        map.values().any(|v| {
            if let Value::String(s) = v {
                s == "red"
            } else {
                false
            }
        })
    } else {
        false
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
                "work" : 98765432100,
                "color" : "red"
            }
        }"#;
        let json: Value = serde_json::from_str(string).unwrap();
        assert_eq!(count_numbers(&json), Some(40));
    }

    #[test]
    fn has_red_test_1() {
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
        assert!(!has_red(&json));
    }

    #[test]
    fn has_red_test_2() {
        let string = r#"{
            "name" : "Jimmy",
            "age" : 30,
            "color" : "red"
        }"#;
        let json: Value = serde_json::from_str(string).unwrap();
        assert!(has_red(&json));
    }
}
