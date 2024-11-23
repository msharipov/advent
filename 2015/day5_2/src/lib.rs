fn contains_double_pair(s: &str) -> bool {
    let pairs = s.chars().zip(s.chars().skip(1));
    let first_pairs = pairs.clone();
    for (i, pair) in first_pairs.enumerate() {
        let mut other_pairs = pairs.clone().skip(i + 2);
        if other_pairs.any(|other| other == pair) {
            return true;
        }
    }
    false
}

fn contains_sandwich(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(2))
        .any(|pair| pair.0 == pair.1)
}

pub fn is_nice(s: &str) -> bool {
    contains_double_pair(s) && contains_sandwich(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_double_pair_test_1() {
        assert!(contains_double_pair("abxab"));
    }

    #[test]
    fn contains_double_pair_test_2() {
        assert!(!contains_double_pair("svaaaunotgbwrtuvnb"));
    }

    #[test]
    fn contains_sandwich_test_1() {
        assert!(contains_sandwich("vsdfonouvfgjndghf"));
    }

    #[test]
    fn contains_sandwich_test_2() {
        assert!(!contains_sandwich("bfgnsuinrfguerbv"));
    }
}
