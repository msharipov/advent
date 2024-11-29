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
    let chars = s.chars().collect::<Vec<_>>();
    let pairs = chars.windows(2);
    for (i, pair) in pairs.clone().enumerate() {
        if pair[0] != pair[1] {
            continue;
        }
        let mut other_pairs = pairs.clone().skip(i + 2);
        if other_pairs.any(|other| other[0] == other[1]) {
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

pub fn next_password(s: &str) -> String {
    let mut pass = s.to_owned();
    loop {
        pass = increment_password(&pass);
        if contains_double_pair(&pass)
            && doesnt_contain_iol(&pass)
            && contains_increasing_triple(&pass)
        {
            return pass;
        }
    }
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
        assert!(contains_double_pair("qvvtneqasddnq"));
    }

    #[test]
    fn contains_double_pair_test_2() {
        assert!(!contains_double_pair("svaaaunotubwrtuvnb"));
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
        assert!(contains_increasing_triple("dnbufgnefgww"));
    }

    #[test]
    fn contains_increasing_triple_test_2() {
        assert!(!contains_increasing_triple("vnierbnvcvdfv"));
    }

    #[test]
    fn next_password_test_1() {
        assert_eq!(next_password("rtvvtbqqrrc"), "rtvvtbqqrsa");
    }
}
