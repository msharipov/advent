use std::num::ParseIntError;

pub fn parse_input(lines: &[&str]) -> Result<i64, ParseIntError> {
    lines[0].parse::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test_1() {
        assert_eq!(parse_input(&["1435873"]), Ok(1435873));
    }

    #[test]
    fn parse_input_test_2() {
        assert!(parse_input(&["abracadabra"]).is_err());
    }
}
