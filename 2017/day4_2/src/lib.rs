use std::collections::{BTreeMap, HashSet};

fn letter_counts(word: &str) -> BTreeMap<char, usize> {
    let mut counts = BTreeMap::new();
    for c in word.chars() {
        match counts.remove(&c) {
            None => counts.insert(c, 1),
            Some(old) => counts.insert(c, old + 1),
        };
    }
    counts
}

pub fn validate_passphrase(passphrase: &str) -> bool {
    let mut found_words = HashSet::new();
    passphrase
        .split(' ')
        .all(|word| found_words.insert(letter_counts(word)))
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

    #[test]
    fn validate_passphrase_test_5() {
        assert!(!validate_passphrase("aba bba cc baa dd ee ff"));
    }

    #[test]
    fn validate_passphrase_test_6() {
        assert!(validate_passphrase("ABCD bb cc abcd dd ee ff"));
    }

    #[test]
    fn validate_passphrase_test_7() {
        assert!(!validate_passphrase("abcadc bb cc cdabca dd ee ff"));
    }

    #[test]
    fn letter_counts_test_1() {
        assert_eq!(letter_counts(""), BTreeMap::new());
    }

    #[test]
    fn letter_counts_test_2() {
        assert_eq!(
            letter_counts("abcd"),
            BTreeMap::from_iter([('a', 1), ('b', 1), ('c', 1), ('d', 1)])
        );
    }

    #[test]
    fn letter_counts_test_3() {
        assert_eq!(
            letter_counts("aabbbbcddd"),
            BTreeMap::from_iter([('a', 2), ('b', 4), ('c', 1), ('d', 3)])
        );
    }
}
