use sscanf::sscanf;
use std::collections::HashMap;

pub fn parse_preferences(lines: &[&str]) -> Result<HashMap<(String, String), i32>, sscanf::Error> {
    let mut prefs = HashMap::new();
    for line in lines {
        let parsed = sscanf!(
            line,
            "{str} would {str:/gain|lose/} {i32} happiness units by sitting next to {str}."
        )?;
        let change = match parsed.1 {
            "gain" => parsed.2,
            "lose" => -parsed.2,
            _ => panic!(),
        };
        prefs.insert((parsed.0.to_owned(), parsed.3.to_owned()), change);
    }
    Ok(prefs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preferences_test_1() {
        let lines = vec![
            "Bob would lose 14 happiness units by sitting next to Alice.",
            "Alice would lose 57 happiness units by sitting next to Bob.",
        ];
        let mut correct = HashMap::new();
        correct.insert(("Alice".to_string(), "Bob".to_string()), -57);
        correct.insert(("Bob".to_string(), "Alice".to_string()), -14);
        assert_eq!(correct, parse_preferences(&lines).unwrap());
    }
}
