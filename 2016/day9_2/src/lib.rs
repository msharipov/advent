use regex::Regex;
use sscanf::sscanf;

pub fn decompress(compressed: &str) -> String {
    let mut decomp = String::new();
    let reg = Regex::new(r"\(\d+x\d+\)").unwrap();
    let matches = reg.find_iter(compressed);
    let mut current_pos = 0;
    for m in matches {
        let start = m.range().start;
        let end = m.range().end;
        if end <= current_pos {
            continue;
        }
        let (len, reps) = sscanf!(m.as_str(), "({usize}x{usize})").unwrap();
        decomp.push_str(&compressed[current_pos..start]);
        for _ in 0..reps {
            decomp.push_str(&compressed[end..end + len]);
        }
        current_pos = end + len;
    }
    decomp.push_str(&compressed[current_pos..]);
    decomp
}

pub fn decompressed_size(compressed: &str) -> usize {
    let mut total: usize = 0;
    let reg = Regex::new(r"\(\d+x\d+\)").unwrap();
    let matches = reg.find_iter(compressed);
    let mut current_pos = 0;
    for m in matches {
        let start = m.range().start;
        let end = m.range().end;
        if end <= current_pos {
            continue;
        }
        let (len, reps) = sscanf!(m.as_str(), "({usize}x{usize})").unwrap();
        total += start - current_pos;
        let base_len = decompressed_size(&compressed[end..end + len]);
        total += base_len * reps;
        current_pos = end + len;
    }
    total + compressed.len() - current_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_test_1() {
        assert_eq!(decompress("ABRACADABRA"), "ABRACADABRA".to_owned());
    }

    #[test]
    fn decompress_test_2() {
        assert_eq!(decompress("ABRA(3x2)CADABRA"), "ABRACADCADABRA".to_owned());
    }

    #[test]
    fn decompress_test_3() {
        assert_eq!(
            decompress("(6x2)(1x3)ABRA(3x2)CADABRA"),
            "(1x3)A(1x3)ABRACADCADABRA".to_owned()
        );
    }

    #[test]
    fn decompressed_size_test_1() {
        assert_eq!(decompressed_size("ABRACADABRA"), 11);
    }

    #[test]
    fn decompressed_size_test_2() {
        assert_eq!(decompressed_size("ABRA(3x2)CADABRA"), 14);
    }

    #[test]
    fn decompressed_size_test_3() {
        assert_eq!(decompressed_size("(6x2)(1x3)ABRA(3x2)CADABRA"), 19);
    }
}
