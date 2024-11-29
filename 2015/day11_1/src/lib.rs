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

fn doesnt_contain_iol(s: &str) -> bool {
    !s.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn contains_increasing_triple(s: &str) -> bool {
    s.bytes()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|w| w[1] == (w[0] + 1) && w[2] == (w[1] + 1))
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

    #[test]
    fn doesnt_contain_iol_test_1() {
        assert!(doesnt_contain_iol("abcdefghjkmnpqrstuvwxyz"));
    }

    #[test]
    fn doesnt_contain_iol_test_2() {
        assert!(!doesnt_contain_iol("diofvnboer"));
    }

    #[test]
    fn contains_increasing_triple_test_1() {
        assert!(contains_increasing_triple("dnbufgnefgww"))
    }

    #[test]
    fn contains_increasing_triple_test_2() {
        assert!(!contains_increasing_triple("vnierbnvcvdfv"))
    }
}
