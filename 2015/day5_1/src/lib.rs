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
}
