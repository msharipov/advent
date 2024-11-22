fn contains_enough_vowels(s: &str) -> bool {
    let mut vowel_count = 0;
    const VOWELS: &str = "aoeiu";
    for c in s.chars() {
        if VOWELS.contains(c) {
            vowel_count += 1;
        }
    }
    vowel_count >= 3
}

fn contains_double_letter(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .any(|pair| pair.0 == pair.1)
}

fn no_naughty_substrings(s: &str) -> bool {
    const NAUGHTY: &[&str] = &["ab", "cd", "pq", "xy"];
    for n in NAUGHTY {
        if s.contains(n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_enough_vowels_test_1() {
        assert!(contains_enough_vowels("afdgndifrpowe"));
    }

    #[test]
    fn contains_enough_vowels_test_2() {
        assert!(!contains_enough_vowels("cmppptelxclwlxz"));
    }

    #[test]
    fn contains_double_letter_test_1() {
        assert!(contains_double_letter("agafdgivdfqwee"));
    }

    #[test]
    fn contains_double_letter_test_2() {
        assert!(!contains_double_letter("aghpngertgbrtnbv"));
    }

    #[test]
    fn no_naughty_substrings_test_1() {
        assert!(no_naughty_substrings("bvnqoerinvdsnfvj"));
    }

    #[test]
    fn no_naughty_substrings_test_2() {
        assert!(!no_naughty_substrings("abcdefghijklmnop"));
    }
}
