pub fn count_chars(s: &str) -> Result<usize, &'static str> {
    let last = s.len() - 1;
    let mut count: usize = 0;
    let chars: Vec<_> = s.chars().collect();
    let mut index: usize = 1;
    while index < last {
        if chars[index] == '\\' {
            if index == last - 1 {
                return Err("invalid escape");
            }
            let next = chars[index + 1];
            if next == '\\' || next == '\"' {
                count += 1;
                index += 2;
                continue;
            }
            if next == 'x' && index + 3 < last {
                count += 1;
                index += 4;
                continue;
            }
            return Err("invalid escape");
        }
        count += 1;
        index += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_chars_test_1() {
        assert_eq!(count_chars("\"abc\\x45\\\\\\\"n\""), Ok(7));
    }

    #[test]
    fn count_chars_test_2() {
        assert_eq!(count_chars("\"ab\\cd\""), Err("invalid escape"));
    }
}
