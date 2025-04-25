use std::num::NonZero;

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

fn stretched_hash(str_to_hash: &str) -> String {
    let mut hash = str_to_hash.to_owned();
    for _ in 0..2017 {
        hash = format!("{:x}", md5::compute(hash));
    }
    hash
}

fn is_key_index(salt: &str, index: u64, computed: &mut Vec<String>) -> bool {
    let composite = format!("{salt}{index}");
    let hash_string = format!("{:x}", md5::compute(composite));
    if let Some(c) = contains_triplet(&hash_string) {
        for i in index + 1..index + 1001 {
            while computed.get(i as usize).is_none() {
                let composite = format!("{salt}{}", computed.len());
                let hash = format!("{:x}", md5::compute(composite));
                computed.push(hash);
            }
            let hash = &computed[i as usize];
            if contains_five_chars(hash, c) {
                return true;
            }
        }
    }
    false
}

pub fn index_of_nth_key(n: NonZero<u64>, salt: &str) -> u64 {
    let n: u64 = n.into();
    let mut index = 0;
    let mut found = 0;
    let mut computed = vec![];
    loop {
        if is_key_index(salt, index, &mut computed) {
            found += 1;
        }
        if found == n {
            break;
        }
        index += 1;
    }
    index
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
        assert!(is_key_index("abc", 39, &mut vec![]));
    }

    #[test]
    fn is_key_index_test_2() {
        assert!(!is_key_index("abc", 18, &mut vec![]));
    }

    #[test]
    fn index_of_nth_key_test_1() {
        assert_eq!(index_of_nth_key(1.try_into().unwrap(), "abc"), 39);
        assert_eq!(index_of_nth_key(2.try_into().unwrap(), "abc"), 92);
    }

    #[test]
    fn stretched_hash_test_1() {
        assert_eq!(stretched_hash("abc0"), "a107ff634856bb300138cac6568c0f24");
    }
}
