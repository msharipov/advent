fn contains_triplet(hash: &str) -> Option<char> {
    for triple in hash.as_bytes().windows(3) {
        if triple[0] == triple[1] && triple[1] == triple[2] {
            return Some(triple[0] as char);
        }
    }
    None
}

fn contains_five_chars(hash: &str, target: char) -> bool {
    for window in hash.as_bytes().windows(5) {
        if window.iter().all(|&c| c as char == target) {
            return true;
        }
    }
    false
}

fn is_key_index(salt: &str, index: u64) -> bool {
    let composite = format!("{salt}{index}");
    let hash_string = format!("{:x}", md5::compute(composite));
    if let Some(c) = contains_triplet(&hash_string) {
        for i in index + 1..index + 1001 {
            let composite = format!("{salt}{i}");
            let hash_string = format!("{:x}", md5::compute(composite));
            if contains_five_chars(&hash_string, c) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_triplet_test_1() {
        assert_eq!(contains_triplet("vdufnvudfniusdfnjnafsdfp"), None);
    }

    #[test]
    fn contains_triplet_test_2() {
        assert_eq!(contains_triplet("vdfvnu3n45334cnasdj23222sad"), Some('2'));
    }

    #[test]
    fn contains_triplet_test_3() {
        assert_eq!(contains_triplet("62m5694mmmk45ml6km4555fdasw"), Some('m'));
    }

    #[test]
    fn contains_five_chars_test_1() {
        assert!(contains_five_chars("adfvnmfffffsdunvb", 'f'));
    }

    #[test]
    fn contains_five_chars_test_2() {
        assert!(!contains_five_chars("adfvnmfffffsdunvb", 'p'));
    }

    #[test]
    fn is_key_index_test_1() {
        assert!(is_key_index("abc", 39));
    }

    #[test]
    fn is_key_index_test_2() {
        assert!(!is_key_index("abc", 18));
    }
}
