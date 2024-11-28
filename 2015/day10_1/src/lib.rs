pub fn generate_next(s: &str) -> String {
    let chars: Vec<_> = s.chars().collect();
    let mut output = String::new();
    let mut current_count = 1;
    let mut current_char = chars[0];
    for pair in chars.windows(2) {
        if pair[0] == pair[1] {
            current_count += 1;
        } else {
            output.push_str(&format!("{}{}", current_count, current_char));
            current_count = 1;
            current_char = pair[1];
        }
    }
    output.push_str(&format!("{}{}", current_count, current_char));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_next_test_1() {
        assert_eq!(generate_next("11122"), "3122".to_string());
    }

    #[test]
    fn generate_next_test_2() {
        assert_eq!(generate_next("1233"), "111223".to_string());
    }
}
