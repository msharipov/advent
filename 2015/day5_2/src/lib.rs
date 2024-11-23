fn contains_double_pair(s: &str) -> bool {
    let mut pairs = s.chars().zip(s.chars().skip(1));
    let mut other_pairs = pairs.clone().skip(2);
    pairs.any(|pair| other_pairs.any(|other| other == pair))
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
        assert!(contains_double_pair("dofhgbinsiotfhun"));
    }

    #[test]
    fn contains_double_pair_test_2() {
        assert!(!contains_double_pair("aaaunotgbwrtuvnb"));
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
