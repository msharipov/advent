fn increment_password(pass: &str) -> String {
    let mut inc = true;
    let mut new: Vec<_> = pass
        .bytes()
        .rev()
        .map(|c| {
            if inc {
                if c == b'z' {
                    return b'a';
                } else {
                    inc = false;
                    return c + 1;
                }
            }
            c
        })
        .collect();
    new.reverse();
    String::from_utf8(new).unwrap()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_password_test_1() {
        assert_eq!(increment_password("vneorvzzz"), "vneorwaaa");
    }

    #[test]
    fn contains_double_pair_test_1() {
        assert!(contains_double_pair("abxab"));
    }

    #[test]
    fn contains_double_pair_test_2() {
        assert!(!contains_double_pair("svaaaunotgbwrtuvnb"));
    }
}
