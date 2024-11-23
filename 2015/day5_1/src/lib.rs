fn contains_enough_vowels(s: &str) -> bool {
    const VOWELS: &str = "aoeiu";
    let vowel_count = s.chars().filter(|&c| VOWELS.contains(c)).count();
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

pub fn is_nice(s: &str) -> bool {
    contains_enough_vowels(s) && contains_double_letter(s) && no_naughty_substrings(s)
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
