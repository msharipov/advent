fn starts_with_6_zeros(key: &str, suffix: u64) -> bool {
    let mut combined = key.to_owned();
    combined.push_str(&suffix.to_string());
    let hash = md5::compute(combined);
    &format!("{:x}", hash)[0..6] == "000000"
}

pub fn find_lowest_suffix(key: &str) -> u64 {
    let mut suffix: u64 = 1;
    loop {
        if starts_with_6_zeros(key, suffix) {
            return suffix;
        }
        suffix += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_5_zeros_test_1() {
        assert!(!starts_with_6_zeros("abcdefg", 234534));
    }
}
