fn contains_double_pair(s: &str) -> bool {
    let mut pairs = s.chars().zip(s.chars().skip(1));
    let mut other_pairs = pairs.clone().skip(2);
    pairs.any(|pair| other_pairs.any(|other| other == pair))
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
    contains_double_pair(s) && contains_double_letter(s) && no_naughty_substrings(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_double_pair_test_1() {
        assert!(contains_double_pair("dofhgbinsiotfhun"));
    }

    #[test]
    fn contains_double_pair_test_2() {
        assert!(!contains_double_pair("aaaunotgbwrtuvnb"));
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
