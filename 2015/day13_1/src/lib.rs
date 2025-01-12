use sscanf::sscanf;
use std::collections::HashMap;

type Preferences = HashMap<(String, String), i32>;

pub fn parse_preferences(lines: &[&str]) -> Result<Preferences, sscanf::Error> {
    let mut prefs = Preferences::new();
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
            "Alice would lose 62 happiness units by sitting next to Carol.",
            "Bob would gain 48 happiness units by sitting next to Carol.",
            "Carol would gain 45 happiness units by sitting next to Bob.",
            "Carol would gain 37 happiness units by sitting next to Alice.",
        ];
        let mut correct = Preferences::new();
        correct.insert(("Alice".to_string(), "Bob".to_string()), -57);
        correct.insert(("Alice".to_string(), "Carol".to_string()), -62);
        correct.insert(("Bob".to_string(), "Alice".to_string()), -14);
        correct.insert(("Bob".to_string(), "Carol".to_string()), 48);
        correct.insert(("Carol".to_string(), "Alice".to_string()), 37);
        correct.insert(("Carol".to_string(), "Bob".to_string()), 45);
        assert_eq!(correct, parse_preferences(&lines).unwrap());
    }

    #[test]
    fn parse_preferences_test_2() {
        let lines = vec![
            "Bob would get 14 happiness units by sitting next to Alice.",
            "Alice would lose 57 happiness units by sitting next to Bob.",
        ];
        assert!(parse_preferences(&lines).is_err());
    }

    #[test]
    fn parse_preferences_test_3() {
        let lines = vec![
            "Bob would gain 14 happiness units by sitting next to Alice.",
            "Alice would lose 5.7 happiness units by sitting next to Bob.",
        ];
        assert!(parse_preferences(&lines).is_err());
    }
}
