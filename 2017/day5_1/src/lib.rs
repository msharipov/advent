use std::num::ParseIntError;

pub fn parse_jumps(lines: &[&str]) -> Result<Vec<i64>, ParseIntError> {
    lines.iter().map(|line| line.parse::<i64>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_jumps_test_1() {
        assert_eq!(
            parse_jumps(&["10", "-56", "0", "39"]),
            Ok(vec![10, -56, 0, 39])
        )
    }
}
