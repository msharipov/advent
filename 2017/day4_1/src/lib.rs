use std::collections::HashSet;

pub fn validate_passphrase(passphrase: &str) -> bool {
    let mut found_words = HashSet::new();
    passphrase.split(' ').all(|word| found_words.insert(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_passphrase_test_1() {
        assert!(validate_passphrase("abcdefg"));
    }

    #[test]
    fn validate_passphrase_test_2() {
        assert!(validate_passphrase("aa bb cc dd ee ff"));
    }

    #[test]
    fn validate_passphrase_test_3() {
        assert!(!validate_passphrase("aa bb cc aa dd ee ff"));
    }

    #[test]
    fn validate_passphrase_test_4() {
        assert!(validate_passphrase("aa bb cc AA dd ee ff"));
    }
}
