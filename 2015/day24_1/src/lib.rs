use std::num::ParseIntError;

pub fn parse_weights(lines: &[&str]) -> Result<Vec<u64>, ParseIntError> {
    lines.iter().map(|&line| line.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_weights_test_1() {
        let parsed = parse_weights(&["1", "2", "3", "4"]);
        assert_eq!(Ok(vec![1, 2, 3, 4]), parsed);
    }

    #[test]
    fn parse_weights_test_2() {
        let parsed = parse_weights(&["1", "-2", "3", "4"]);
        assert!(parsed.is_err());
    }
}
